use crate::{
    annotated,
    ast::{expression::Expression, scope::Scope, SemanticAnalysisError},
    lexer, next_token,
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
};

pub fn parse(stream: &mut Stream, identifier: &str) -> Result<(Expression, Token), ParserError> {
    let member = next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()});

    let next_token = match lexer::next_token(stream)? {
        Some(token) => token,
        None => return Err(ParserError::UnexpectedEOF),
    };

    Ok((
        Expression::MemberAccess(identifier.to_owned(), member),
        next_token,
    ))
}

pub fn semantic_analysis(
    scope: &Scope,
    variable_name: String,
    member_name: String,
) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
    // Verify variable and member exist
    let structure = scope.get_variable(&variable_name)?;
    for (name, _) in structure.members() {
        if *name == member_name {
            return Ok(annotated::expression::Expression::MemberAccess(
                variable_name,
                member_name,
            ));
        }
    }

    Err(SemanticAnalysisError::InvalidMember(
        variable_name,
        member_name,
    ))
}
