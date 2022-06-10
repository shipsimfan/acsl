use crate::{
    ast::{
        declaration::{function::parse_function, structure::parse_struct},
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

pub fn parse(code: &str) -> Result<AbstractSyntaxTree, ParserError> {
    println!("Tokens:");

    let mut ast = AbstractSyntaxTree::new();
    let mut stream = Stream::new(code);

    while let Some(token) = lexer::next_token(&mut stream)? {
        ast.push(match token.class() {
            TokenClass::Fn => parse_function(&mut stream)?,
            TokenClass::Struct => parse_struct(&mut stream)?,
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
