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

pub fn parse_constant_buffer(stream: &mut Stream) -> Result<Declaration, ParserError> {
    let name = next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()});

    next_token!(stream, TokenClass::Colon => {});

    let type_name =
        next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()});

    next_token!(stream, TokenClass::Colon => {});

    let slot = next_token!(stream, TokenClass::IntegerLiteral(slot) => {*slot});

    next_token!(stream, TokenClass::SemiColon => {});

    Ok(Declaration::ConstantBuffer(name, slot, type_name))
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    name: String,
    slot: usize,
    type_name: String,
) -> Result<annotated::constant_buffer::ConstantBuffer, SemanticAnalysisError> {
    annotated::constant_buffer::ConstantBuffer::new(
        name,
        slot,
        Type::from_name(&type_name, output_tree)?,
    )
}
