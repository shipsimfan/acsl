use crate::{
    annotated,
    ast::{expression::Expression, scope::Scope, SemanticAnalysisError},
    parser::ParserError,
    tokens::Token,
};

pub fn parse(identifier: &str, next_token: Token) -> Result<(Expression, Token), ParserError> {
    Ok((Expression::Variable(identifier.to_owned()), next_token))
}

pub fn semantic_analysis(
    scope: &Scope,
    variable: String,
) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
    scope.get_variable(&variable)?;
    Ok(annotated::expression::Expression::Variable(variable))
}
