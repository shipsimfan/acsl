use super::Statement;
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::{expression::Expression, scope::Scope, SemanticAnalysisError},
    next_token,
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
};

pub fn parse(stream: &mut Stream) -> Result<Statement, ParserError> {
    let name = next_token!(stream, TokenClass::Identifier(identifier) => {identifier.clone()});

    next_token!(stream, TokenClass::Equal => {});

    let (expression, next_token) = Expression::parse(stream)?;

    match next_token.class() {
        TokenClass::SemiColon => Ok(Statement::VariableDefinition(name, expression)),
        _ => Err(ParserError::UnexpectedToken(next_token)),
    }
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    scope: &mut Scope,
    name: String,
    expression: Expression,
) -> Result<annotated::statement::Statement, SemanticAnalysisError> {
    let expression_type = expression.get_type(output_tree, scope)?;

    scope.define_variable(name.clone(), expression_type.clone())?;

    Ok(annotated::statement::Statement::VariableDefinition(
        name,
        expression.semantic_analysis(output_tree, scope)?,
        expression_type,
    ))
}
