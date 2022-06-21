use super::Declaration;
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::{expression::Expression, SemanticAnalysisError},
    next_token,
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
};

pub fn parse_constant(stream: &mut Stream) -> Result<Declaration, ParserError> {
    let name = next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()});

    next_token!(stream, TokenClass::Equal => {});

    let (expression, next_token) = Expression::parse(stream)?;

    match next_token.class() {
        TokenClass::SemiColon => Ok(Declaration::Constant(name, expression)),
        _ => Err(ParserError::UnexpectedToken(next_token)),
    }
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    name: String,
    expression: Expression,
) -> Result<annotated::constant::Constant, SemanticAnalysisError> {
    let constant_type = expression.get_type(output_tree, output_tree.global_scope())?;

    Ok(annotated::constant::Constant::new(
        name,
        expression.semantic_analysis(output_tree, output_tree.global_scope())?,
        constant_type,
    ))
}
