use super::Declaration;
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::SemanticAnalysisError,
    next_token,
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
    types::Type,
};

pub fn parse_texture(stream: &mut Stream) -> Result<Declaration, ParserError> {
    let (name, texture_type) = next_token!(stream,
        TokenClass::Identifier(identifier) => {(identifier.to_owned(), "float4".to_owned())},
        TokenClass::LeftAngleBracket => {
            let texture_type = next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()});
            next_token!(stream, TokenClass::RightAngleBracket => {});
            (next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()}), texture_type)
        }
    );

    next_token!(stream, TokenClass::Colon => {});

    let slot = next_token!(stream, TokenClass::IntegerLiteral(slot) => {*slot});

    next_token!(stream, TokenClass::SemiColon => {});

    Ok(Declaration::Texture(name, slot, texture_type))
}

pub fn semantic_analysis(
    name: String,
    slot: usize,
    texture_type: String,
    output_tree: &AnnotatedSyntaxTree,
) -> Result<annotated::texture::Texture, SemanticAnalysisError> {
    let texture_type = Type::from_name(&texture_type, &output_tree)?;

    if !texture_type.is_float_vector() && !texture_type.is_uint() && !texture_type.is_float() {
        return Err(SemanticAnalysisError::InvalidTextureType(
            texture_type.to_string(),
        ));
    }

    annotated::texture::Texture::new(name, slot, texture_type)
}
