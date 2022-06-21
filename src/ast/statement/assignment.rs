use super::Statement;
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::{expression::Expression, scope::Scope, SemanticAnalysisError},
    next_token,
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
};

pub fn parse(stream: &mut Stream, name: &str) -> Result<Statement, ParserError> {
    next_token!(stream, TokenClass::Equal => {});

    let (expression, next_token) = Expression::parse(stream)?;

    match next_token.class() {
        TokenClass::SemiColon => Ok(Statement::Assignment(name.to_owned(), expression)),
        _ => Err(ParserError::UnexpectedToken(next_token)),
    }
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    scope: &Scope,
    name: String,
    expression: Expression,
) -> Result<annotated::statement::Statement, SemanticAnalysisError> {
    // Verify the variable has been defined
    let variable_type = scope.get_variable(&name)?;

    // Verify the type
    let expression_type = expression.get_type(output_tree, scope)?;
    if expression_type != *variable_type {
        Err(SemanticAnalysisError::VariableTypeMismatch(
            name,
            expression_type.to_string(),
            variable_type.to_string(),
        ))
    } else {
        Ok(annotated::statement::Statement::Assignment(
            name,
            expression.semantic_analysis(output_tree, scope)?,
        ))
    }
}
