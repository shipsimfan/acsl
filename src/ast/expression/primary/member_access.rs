use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::{expression::Expression, scope::Scope, SemanticAnalysisError},
    lexer, next_token,
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
};

pub fn parse(
    stream: &mut Stream,
    expression: Expression,
) -> Result<(Expression, Token), ParserError> {
    let member = next_token!(stream, TokenClass::Identifier(identifier) => {identifier.to_owned()});

    let next_token = match lexer::next_token(stream)? {
        Some(token) => token,
        None => return Err(ParserError::UnexpectedEOF),
    };

    Ok((
        Expression::MemberAccess(Box::new(expression), member),
        next_token,
    ))
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    scope: &Scope,
    expression: Box<Expression>,
    member_name: String,
) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
    // Verify variable and member exist
    let structure = expression.get_type(output_tree, scope)?;
    for (name, member_type) in structure.members() {
        if *name == member_name {
            return Ok(annotated::expression::Expression::MemberAccess(
                Box::new(expression.semantic_analysis(output_tree, scope)?),
                member_name,
                member_type.clone(),
            ));
        }
    }

    Err(SemanticAnalysisError::InvalidMember(
        structure.to_string(),
        member_name,
    ))
}
