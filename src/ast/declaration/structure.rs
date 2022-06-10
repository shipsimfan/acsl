use super::Declaration;
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::SemanticAnalysisError,
    lexer,
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
    types::Type,
};

pub fn parse_struct(stream: &mut Stream) -> Result<Declaration, ParserError> {
    let name = match lexer::next_token(stream)? {
        Some(token) => match token.class() {
            TokenClass::Identifier(identifier) => identifier.to_owned(),
            _ => return Err(ParserError::UnexpectedToken(token)),
        },
        None => return Err(ParserError::UnexpectedEOF),
    };

    match lexer::next_token(stream)? {
        Some(token) => match token.class() {
            TokenClass::OpenCurlyBrace => {}
            _ => return Err(ParserError::UnexpectedToken(token)),
        },
        None => return Err(ParserError::UnexpectedEOF),
    }

    let mut members = Vec::new();
    loop {
        let name = match lexer::next_token(stream)? {
            Some(token) => match token.class() {
                TokenClass::Identifier(identifier) => identifier.to_owned(),
                TokenClass::CloseCurlyBrace => break,
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => return Err(ParserError::UnexpectedEOF),
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
                TokenClass::Identifier(identifier) => identifier.to_owned(),
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => return Err(ParserError::UnexpectedEOF),
        };

        match lexer::next_token(stream)? {
            Some(token) => match token.class() {
                TokenClass::CloseCurlyBrace => {
                    members.push((name, type_name, None));
                    break;
                }
                TokenClass::Comma => {
                    members.push((name, type_name, None));
                    continue;
                }
                TokenClass::Colon => {}
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => return Err(ParserError::UnexpectedEOF),
        }

        let semantic = match lexer::next_token(stream)? {
            Some(token) => match token.class() {
                TokenClass::Identifier(identifier) => identifier.to_owned(),
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => return Err(ParserError::UnexpectedEOF),
        };

        members.push((name, type_name, Some(semantic)));

        match lexer::next_token(stream)? {
            Some(token) => match token.class() {
                TokenClass::CloseCurlyBrace => break,
                TokenClass::Comma => {}
                _ => return Err(ParserError::UnexpectedToken(token)),
            },
            None => return Err(ParserError::UnexpectedEOF),
        }
    }

    Ok(Declaration::Struct(name, members))
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    name: String,
    members: Vec<(String, String, Option<String>)>,
) -> Result<annotated::structure::Struct, SemanticAnalysisError> {
    // Verify unique names & types
    let mut s_members = Vec::with_capacity(members.len());
    let mut semantics: Option<Vec<String>> = None;

    for i in 0..members.len() {
        for j in i + 1..members.len() {
            if members[j].0 == members[i].0 {
                return Err(SemanticAnalysisError::MultipleDefinition(name));
            }
        }

        let member_type = Type::from_name(&members[i].1, output_tree)?;

        s_members.push((members[i].0.clone(), member_type));

        match &members[i].2 {
            Some(semantic) => {
                if i > 0 && semantics.is_none() {
                    return Err(SemanticAnalysisError::AllFieldsNeedSemantics(name));
                }

                match &mut semantics {
                    Some(semantics) => semantics.push(semantic.to_owned()),
                    None => semantics = Some(vec![semantic.to_owned()]),
                }
            }
            None => {
                if semantics.is_some() {
                    return Err(SemanticAnalysisError::AllFieldsNeedSemantics(name));
                }
            }
        }
    }

    Ok(annotated::structure::Struct::new(
        name, s_members, semantics,
    ))
}
