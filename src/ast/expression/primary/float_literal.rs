use crate::{
    annotated,
    ast::{expression::Expression, SemanticAnalysisError},
    lexer,
    parser::ParserError,
    stream::Stream,
    tokens::Token,
};

pub fn parse(stream: &mut Stream, value: f64) -> Result<(Expression, Token), ParserError> {
    match lexer::next_token(stream)? {
        Some(next_token) => Ok((Expression::FloatLiteral(value), next_token)),
        None => return Err(ParserError::UnexpectedEOF),
    }
}

pub fn semantic_analysis(
    value: f64,
) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
    Ok(annotated::expression::Expression::FloatLiteral(value))
}
