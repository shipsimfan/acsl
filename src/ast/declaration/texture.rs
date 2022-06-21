use super::Declaration;
use crate::{
    annotated::{self},
    ast::SemanticAnalysisError,
    next_token,
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
};

pub fn parse_texture(stream: &mut Stream) -> Result<Declaration, ParserError> {
    let name = next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()});

    next_token!(stream, TokenClass::Colon => {});

    let slot = next_token!(stream, TokenClass::IntegerLiteral(slot) => {*slot});

    next_token!(stream, TokenClass::SemiColon => {});

    Ok(Declaration::Texture(name, slot))
}

pub fn semantic_analysis(
    name: String,
    slot: usize,
) -> Result<annotated::texture::Texture, SemanticAnalysisError> {
    annotated::texture::Texture::new(name, slot)
}
