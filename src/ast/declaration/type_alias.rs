use super::Declaration;
use crate::{
    annotated::AnnotatedSyntaxTree, ast::SemanticAnalysisError, next_token, parser::ParserError,
    stream::Stream, tokens::TokenClass, types::Type,
};

pub fn parse_type_alias(stream: &mut Stream) -> Result<Declaration, ParserError> {
    let name = next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()});

    next_token!(stream, TokenClass::Equal => {});

    let type_name =
        next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()});

    next_token!(stream, TokenClass::SemiColon => {});

    Ok(Declaration::TypeAlias(name, type_name))
}

pub fn semantic_analysis(
    output_tree: &mut AnnotatedSyntaxTree,
    name: String,
    type_name: String,
) -> Result<(), SemanticAnalysisError> {
    output_tree.push_type_alias(name, Type::alias(Type::from_name(&type_name, output_tree)?))
}
