use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::{expression::Expression, scope::Scope, SemanticAnalysisError},
    lexer, next_token,
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
    types::Type,
};

pub fn parse(stream: &mut Stream, identifier: &str) -> Result<(Expression, Token), ParserError> {
    let mut members = Vec::new();

    loop {
        let name = next_token!(stream,
            TokenClass::Identifier(identifier) => {identifier.to_owned()},
            TokenClass::CloseCurlyBrace => {break}
        );

        next_token!(stream, TokenClass::Colon => {});

        let (expression, next_token) = Expression::parse(stream)?;

        members.push((name, expression));

        match next_token.class() {
            TokenClass::CloseCurlyBrace => break,
            TokenClass::Comma => {}
            _ => return Err(ParserError::UnexpectedToken(next_token)),
        }
    }

    let next_token = match lexer::next_token(stream)? {
        Some(token) => token,
        None => return Err(ParserError::UnexpectedEOF),
    };

    Ok((
        Expression::StructCreation(identifier.to_owned(), members),
        next_token,
    ))
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    scope: &Scope,
    name: String,
    mut members: Vec<(String, Expression)>,
) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
    let structure = output_tree.get_structure(&name)?;

    let mut s_members = Vec::new();

    for (member_name, member_type) in structure.members() {
        // Locate defined value
        let mut index = None;
        for i in 0..members.len() {
            if members[i].0 == *member_name {
                index = Some(i);
                break;
            }
        }

        let (_, expression) = match index {
            Some(index) => members.remove(index),
            None => {
                return Err(SemanticAnalysisError::MissingStructureMember(
                    name,
                    member_name.clone(),
                ))
            }
        };

        // Verify type
        let e_type = expression.get_type(output_tree, scope)?;
        if *member_type != e_type {
            return Err(SemanticAnalysisError::InvalidMemberType(
                name,
                member_name.to_string(),
                e_type.to_string(),
                member_type.to_string(),
            ));
        }

        // Evaluate expression
        let expression = expression.semantic_analysis(output_tree, scope)?;

        // Insert member
        s_members.push(expression);
    }

    Ok(annotated::expression::Expression::StructCreation(
        name,
        s_members,
        Type::Struct(structure.clone()),
    ))
}
