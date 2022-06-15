#[derive(Debug)]
pub enum TokenClass {
    Fn,
    Struct,
    Return,
    CBuffer,
    Identifier(String),
    FloatLiteral(f64),
    IntegerLiteral(usize),
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Dash,
    RightArrow,
    SemiColon,
    Colon,
    Period,
    Comma,
    LeftAngleBracket,
    RightAngleBracket,
    Asterick,
}

#[derive(Debug)]
pub struct Token {
    class: TokenClass,
    column: usize,
    line: usize,
}

impl Token {
    pub fn new(class: TokenClass, column: usize, line: usize) -> Self {
        Token {
            class,
            column,
            line,
        }
    }

    pub fn class(&self) -> &TokenClass {
        &self.class
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}:{}", self.class, self.line, self.column)
    }
}

impl std::fmt::Display for TokenClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenClass::Fn => write!(f, "fn"),
            TokenClass::Struct => write!(f, "struct"),
            TokenClass::Return => write!(f, "return"),
            TokenClass::CBuffer => write!(f, "cbuffer"),
            TokenClass::Identifier(identifier) => write!(f, "\"{}\"", identifier),
            TokenClass::FloatLiteral(value) => write!(f, "{}", value),
            TokenClass::IntegerLiteral(value) => write!(f, "{}", value),
            TokenClass::OpenParenthesis => write!(f, "("),
            TokenClass::CloseParenthesis => write!(f, ")"),
            TokenClass::OpenCurlyBrace => write!(f, "{{"),
            TokenClass::CloseCurlyBrace => write!(f, "}}"),
            TokenClass::Dash => write!(f, "-"),
            TokenClass::RightArrow => write!(f, "->"),
            TokenClass::SemiColon => write!(f, ";"),
            TokenClass::Colon => write!(f, ":"),
            TokenClass::Period => write!(f, "."),
            TokenClass::Comma => write!(f, ","),
            TokenClass::LeftAngleBracket => write!(f, "<"),
            TokenClass::RightAngleBracket => write!(f, ">"),
            TokenClass::Asterick => write!(f, "*"),
        }
    }
}
