use super::Statement;
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::{expression::Expression, scope::Scope, SemanticAnalysisError},
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
    types::Type,
};

pub fn parse(stream: &mut Stream) -> Result<Statement, ParserError> {
    let (expression, next_token) = Expression::parse(stream)?;

    match next_token.class() {
        TokenClass::SemiColon => Ok(Statement::Return(expression)),
        _ => return Err(ParserError::UnexpectedToken(next_token)),
    }
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    scope: &mut Scope,
    function_return_type: &Type,
    expression: Expression,
) -> Result<annotated::statement::Statement, SemanticAnalysisError> {
    let expression_type = expression.get_type(output_tree, scope)?;
    if expression_type == *function_return_type {
        Ok(annotated::statement::Statement::Return(
            expression.semantic_analysis(output_tree, scope)?,
            expression_type,
        ))
    } else {
        Err(SemanticAnalysisError::InvalidReturnType(
            expression_type.to_string(),
            function_return_type.to_string(),
        ))
    }
}
