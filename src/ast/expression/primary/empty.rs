use crate::{
    annotated::{self},
    ast::{expression::Expression, SemanticAnalysisError},
    parser::ParserError,
    tokens::Token,
};

pub fn parse(token: Token) -> Result<(Expression, Token), ParserError> {
    Ok((Expression::Empty, token))
}

pub fn semantic_analysis() -> Result<annotated::expression::Expression, SemanticAnalysisError> {
    Ok(annotated::expression::Expression::Empty)
}
