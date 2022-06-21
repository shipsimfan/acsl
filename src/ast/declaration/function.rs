use super::Declaration;
use crate::{
    annotated::{self, function::FunctionParameter, AnnotatedSyntaxTree},
    ast::{code_block::CodeBlock, SemanticAnalysisError},
    lexer, next_token,
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
    types::Type,
};

pub fn parse_function(stream: &mut Stream) -> Result<Declaration, ParserError> {
    let name = next_token!(stream, TokenClass::Identifier(name) => { name.to_owned() });

    // Parse parameters
    next_token!(stream, TokenClass::OpenParenthesis => {});

    let mut parameters = Vec::new();
    let mut name_token = match lexer::next_token(stream)? {
        Some(token) => match token.class() {
            TokenClass::Identifier(_) => Some(token),
            TokenClass::CloseParenthesis => None,
            _ => return Err(ParserError::UnexpectedToken(token)),
        },
        None => return Err(ParserError::UnexpectedEOF),
    };

    loop {
        let (name, mutable) = match name_token.take() {
            Some(token) => match token.class() {
                TokenClass::Identifier(name) => (name.to_owned(), false),
                TokenClass::Mut => (
                    next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()}),
                    true,
                ),
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => break,
        };

        next_token!(stream, TokenClass::Colon => {});

        let type_name =
            next_token!(stream, TokenClass::Identifier(type_name) => {type_name.to_owned()});

        parameters.push((name, type_name, mutable));

        next_token!(stream,
            TokenClass::CloseParenthesis => {break},
            TokenClass::Comma => {name_token = lexer::next_token(stream)?}
        );
    }

    // Parse return type
    let return_type = next_token!(stream,
        TokenClass::OpenCurlyBrace => {None},
        TokenClass::RightArrow => {
            next_token!(stream, TokenClass::Identifier(return_type) => {
                next_token!(stream, TokenClass::OpenCurlyBrace => {Some(return_type.to_owned())})
            })
        }
    );

    // Parse code block
    let code_block = CodeBlock::parse(stream, 1)?;

    Ok(Declaration::Function(
        name,
        parameters,
        return_type,
        code_block,
    ))
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    name: String,
    parameters: Vec<(String, String, bool)>,
    return_type: Option<String>,
    code_block: CodeBlock,
) -> Result<annotated::function::Function, SemanticAnalysisError> {
    let mut f_parameters = Vec::with_capacity(parameters.len());

    let mut scope = output_tree.global_scope().new_child();
    for (name, type_name, mutable) in parameters {
        let parameter_type = Type::from_name(&type_name, output_tree)?;
        f_parameters.push(FunctionParameter::new(name.clone(), parameter_type.clone()));
        scope.define_variable(name, parameter_type, mutable)?;
    }

    let return_type = match &return_type {
        Some(return_type) => Type::from_name(return_type, output_tree)?,
        None => Type::void(),
    };

    let code_block = code_block.semantic_analysis(output_tree, &mut scope, &return_type)?;

    Ok(annotated::function::Function::new(
        name,
        f_parameters,
        return_type,
        code_block,
    ))
}
