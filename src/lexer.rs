use crate::{
    stream::Stream,
    tokens::{Token, TokenClass},
};

#[derive(Debug)]
pub enum LexerError {
    UnknownCharacter(char, usize, usize),
    NoExponentialDigits(usize, usize),
}

fn parse_identifier(stream: &mut Stream) -> String {
    let mut string = stream.current_char().unwrap().to_string();

    while let Some(c) = stream.next_char() {
        if !c.is_alphanumeric() && c != '_' {
            break;
        }

        string.push(c);
        stream.next();
    }

    string
}

fn parse_number(stream: &mut Stream) -> Result<TokenClass, LexerError> {
    let mut value = stream.current_char().unwrap().to_digit(10).unwrap() as usize;

    while let Some(c) = stream.next_char() {
        if c.is_digit(10) {
            value *= 10;
            value += c.to_digit(10).unwrap() as usize;
        } else if c == '.' {
            stream.next();
            return parse_fractional(stream, value);
        } else if c == 'e' {
            stream.next();
            return parse_exponent(stream, value, 0);
        } else {
            break;
        }

        stream.next();
    }

    Ok(TokenClass::IntegerLiteral(value))
}

fn parse_fractional(stream: &mut Stream, integer: usize) -> Result<TokenClass, LexerError> {
    let mut fractional = 0;

    while let Some(c) = stream.next_char() {
        if c.is_digit(10) {
            fractional *= 10;
            fractional += c.to_digit(10).unwrap() as usize;
        } else if c == 'e' {
            stream.next();
            return parse_exponent(stream, integer, fractional);
        } else {
            break;
        }

        stream.next();
    }

    Ok(TokenClass::FloatLiteral(
        format!("{}.{}", integer, fractional).parse().unwrap(),
    ))
}

fn parse_exponent(
    stream: &mut Stream,
    integer: usize,
    fractional: usize,
) -> Result<TokenClass, LexerError> {
    match stream.next_char() {
        Some(c) => match c.is_digit(10) {
            true => {}
            false => {
                return Err(LexerError::NoExponentialDigits(
                    stream.column(),
                    stream.line(),
                ))
            }
        },
        None => {
            return Err(LexerError::NoExponentialDigits(
                stream.column(),
                stream.line(),
            ))
        }
    }

    let mut exponential = 0;

    while let Some(c) = stream.next_char() {
        if c.is_digit(10) {
            exponential *= 10;
            exponential += c.to_digit(10).unwrap() as usize;
        } else {
            break;
        }

        stream.next();
    }

    Ok(TokenClass::FloatLiteral(
        format!("{}.{}e{}", integer, fractional, exponential)
            .parse()
            .unwrap(),
    ))
}

pub fn next_token(stream: &mut Stream) -> Result<Option<Token>, LexerError> {
    while let Some(c) = stream.next() {
        if !c.is_whitespace() {
            break;
        }
    }

    let column = stream.column();
    let line = stream.line();

    let token_class = match stream.current_char() {
        Some(c) => {
            if c.is_alphabetic() || c == '_' {
                let identifier = parse_identifier(stream);

                match identifier.as_str() {
                    "fn" => TokenClass::Fn,
                    "struct" => TokenClass::Struct,
                    "return" => TokenClass::Return,
                    "cbuffer" => TokenClass::CBuffer,
                    "const" => TokenClass::Const,
                    "type" => TokenClass::Type,
                    "let" => TokenClass::Let,
                    "mut" => TokenClass::Mut,
                    _ => TokenClass::Identifier(identifier),
                }
            } else if c.is_digit(10) {
                parse_number(stream)?
            } else if c == '.' {
                match stream.next_char() {
                    Some(c) => match c.is_digit(10) {
                        true => parse_fractional(stream, 0)?,
                        false => TokenClass::Period,
                    },
                    None => TokenClass::Period,
                }
            } else {
                match c {
                    '(' => TokenClass::OpenParenthesis,
                    ')' => TokenClass::CloseParenthesis,
                    '{' => TokenClass::OpenCurlyBrace,
                    '}' => TokenClass::CloseCurlyBrace,
                    '-' => match stream.next_char() {
                        Some(c) => match c {
                            '>' => {
                                stream.next();
                                TokenClass::RightArrow
                            }
                            _ => TokenClass::Dash,
                        },
                        None => TokenClass::Dash,
                    },
                    ';' => TokenClass::SemiColon,
                    ':' => TokenClass::Colon,
                    ',' => TokenClass::Comma,
                    '<' => TokenClass::LeftAngleBracket,
                    '>' => TokenClass::RightAngleBracket,
                    '=' => TokenClass::Equal,
                    '*' => TokenClass::Asterick,
                    '+' => TokenClass::Plus,
                    _ => return Err(LexerError::UnknownCharacter(c, column, line)),
                }
            }
        }
        None => return Ok(None),
    };

    let token = Token::new(token_class, column, line);
    Ok(Some(token))
}

impl std::error::Error for LexerError {}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnknownCharacter(c, column, line) => {
                write!(f, "Unknown character '{}' at {}:{}", c, line, column)
            }
            LexerError::NoExponentialDigits(line, column) => {
                write!(f, "Exponent has no digits at {}:{}", line, column)
            }
        }
    }
}
