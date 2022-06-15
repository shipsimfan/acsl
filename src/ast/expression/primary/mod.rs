use super::Expression;
use crate::{
    lexer,
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
};

pub mod empty;
pub mod float_literal;
pub mod function_call;
pub mod member_access;
pub mod struct_creation;
pub mod variable;

pub fn parse(stream: &mut Stream) -> Result<(Expression, Token), ParserError> {
    match lexer::next_token(stream)? {
        Some(token) => match token.class() {
            TokenClass::OpenParenthesis => {
                let (expression, next_token) = Expression::parse(stream)?;
                match next_token.class() {
                    TokenClass::CloseParenthesis => {}
                    _ => return Err(ParserError::UnexpectedToken(next_token)),
                }

                match lexer::next_token(stream)? {
                    Some(token) => Ok((expression, token)),
                    None => Err(ParserError::UnexpectedEOF),
                }
            }
            TokenClass::FloatLiteral(value) => float_literal::parse(stream, *value),
            TokenClass::Identifier(identifier) => match lexer::next_token(stream)? {
                Some(next_token) => match next_token.class() {
                    TokenClass::OpenParenthesis => function_call::parse(stream, identifier),
                    TokenClass::OpenCurlyBrace => struct_creation::parse(stream, identifier),
                    TokenClass::Period => member_access::parse(stream, identifier),
                    _ => variable::parse(identifier, next_token),
                },
                None => Err(ParserError::UnexpectedEOF),
            },
            _ => empty::parse(token),
        },
        None => Err(ParserError::UnexpectedEOF),
    }
}
