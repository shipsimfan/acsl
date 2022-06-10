use super::Declaration;
use crate::{
    annotated::{self, function::FunctionParameter, AnnotatedSyntaxTree},
    ast::{code_block::CodeBlock, scope::Scope, SemanticAnalysisError},
    lexer,
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
    types::Type,
};

pub fn parse_function(stream: &mut Stream) -> Result<Declaration, ParserError> {
    // Parse name
    let name = match lexer::next_token(stream)? {
        Some(token) => match token.class() {
            TokenClass::Identifier(name) => name.to_owned(),
            _ => return Err(ParserError::UnexpectedToken(token)),
        },
        None => return Err(ParserError::UnexpectedEOF),
    };

    // Parse parameters
    match lexer::next_token(stream)? {
        Some(token) => match token.class() {
            TokenClass::OpenParenthesis => {}
            _ => return Err(ParserError::UnexpectedToken(token)),
        },
        None => return Err(ParserError::UnexpectedEOF),
    };

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
        let name = match name_token.take() {
            Some(token) => match token.class() {
                TokenClass::Identifier(name) => name.to_owned(),
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => break,
        };

        match lexer::next_token(stream)? {
            Some(token) => match token.class() {
                TokenClass::Colon => {}
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => return Err(ParserError::UnexpectedEOF),
        }

        let type_name = match lexer::next_token(stream)? {
            Some(token) => match token.class() {
                TokenClass::Identifier(type_name) => type_name.to_owned(),
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => return Err(ParserError::UnexpectedEOF),
        };

        parameters.push((name, type_name));

        match lexer::next_token(stream)? {
            Some(token) => match token.class() {
                TokenClass::CloseParenthesis => break,
                TokenClass::Comma => name_token = lexer::next_token(stream)?,
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => return Err(ParserError::UnexpectedEOF),
        }
    }

    // Parse return type
    let return_type = match lexer::next_token(stream)? {
        Some(token) => match token.class() {
            TokenClass::OpenCurlyBrace => None,
            TokenClass::RightArrow => match lexer::next_token(stream)? {
                Some(token) => match token.class() {
                    TokenClass::Identifier(return_type) => match lexer::next_token(stream)? {
                        Some(token) => match token.class() {
                            TokenClass::OpenCurlyBrace => Some(return_type.to_owned()),
                            _ => return Err(ParserError::UnexpectedToken(token)),
                        },
                        None => return Err(ParserError::UnexpectedEOF),
                    },
                    _ => return Err(ParserError::UnexpectedToken(token)),
                },
                None => return Err(ParserError::UnexpectedEOF),
            },
            _ => return Err(ParserError::UnexpectedToken(token)),
        },
        None => return Err(ParserError::UnexpectedEOF),
    };

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
    parameters: Vec<(String, String)>,
    return_type: Option<String>,
    code_block: CodeBlock,
) -> Result<annotated::function::Function, SemanticAnalysisError> {
    let mut f_parameters = Vec::with_capacity(parameters.len());

    let mut scope = Scope::new();
    for (name, type_name) in parameters {
        let parameter_type = Type::from_name(&type_name, output_tree)?;
        f_parameters.push(FunctionParameter::new(name.clone(), parameter_type.clone()));
        scope.define_variable(name, parameter_type)?;
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
