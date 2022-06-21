use crate::{
    ast::{
        declaration::{
            constant::parse_constant, constant_buffer::parse_constant_buffer,
            function::parse_function, structure::parse_struct, texture::parse_texture,
            type_alias::parse_type_alias,
        },
        AbstractSyntaxTree,
    },
    lexer,
    stream::Stream,
    tokens::{Token, TokenClass},
};

#[derive(Debug)]
pub enum ParserError {
    LexerError(lexer::LexerError),
    UnexpectedToken(Token),
    UnexpectedEOF,
}

#[macro_export]
macro_rules! next_token {
    ($stream:expr, $($token_class: pat => $block:block),+) => {
        match crate::lexer::next_token($stream)? {
            Some(token) => match token.class() {
                $($token_class => $block),+
                _ => return Err(crate::parser::ParserError::UnexpectedToken(token)),
            }
            None => return Err(crate::parser::ParserError::UnexpectedEOF),
        }
    };
}

pub fn parse(code: &str) -> Result<AbstractSyntaxTree, ParserError> {
    println!("Tokens:");

    let mut ast = AbstractSyntaxTree::new();
    let mut stream = Stream::new(code);

    while let Some(token) = lexer::next_token(&mut stream)? {
        ast.push(match token.class() {
            TokenClass::Fn => parse_function(&mut stream)?,
            TokenClass::Struct => parse_struct(&mut stream)?,
            TokenClass::CBuffer => parse_constant_buffer(&mut stream)?,
            TokenClass::Type => parse_type_alias(&mut stream)?,
            TokenClass::Const => parse_constant(&mut stream)?,
            TokenClass::Identifier(identifier) => match identifier.as_str() {
                "texture" => parse_texture(&mut stream)?,
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            _ => return Err(ParserError::UnexpectedToken(token)),
        })
    }

    println!("\n{}", ast);

    Ok(ast)
}

impl std::error::Error for ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::LexerError(error) => write!(f, "{}", error),
            ParserError::UnexpectedToken(token) => write!(f, "Unexpected token {}", token),
            ParserError::UnexpectedEOF => write!(f, "Unexpected end of file"),
        }
    }
}

impl From<lexer::LexerError> for ParserError {
    fn from(error: lexer::LexerError) -> Self {
        ParserError::LexerError(error)
    }
}
