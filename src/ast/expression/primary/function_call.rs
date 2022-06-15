use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::{expression::Expression, scope::Scope, SemanticAnalysisError},
    lexer,
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
};

pub fn parse(stream: &mut Stream, identifier: &str) -> Result<(Expression, Token), ParserError> {
    let mut parameters = Vec::new();

    loop {
        let (parameter, next_token) = Expression::parse(stream)?;

        parameters.push(parameter);

        match next_token.class() {
            TokenClass::Comma => {}
            TokenClass::CloseParenthesis => break,
            _ => return Err(ParserError::UnexpectedToken(next_token)),
        }
    }

    match lexer::next_token(stream)? {
        Some(token) => Ok((
            Expression::FunctionCall(identifier.to_owned(), parameters),
            token,
        )),
        None => Err(ParserError::UnexpectedEOF),
    }
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    scope: &Scope,
    name: String,
    parameters: Vec<Expression>,
) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
    // Verify function existance
    let function = output_tree.get_function(&name)?;

    // Verify parameter count
    if function.parameters().len() != parameters.len() {
        return Err(SemanticAnalysisError::InvalidParameterCount(
            name,
            parameters.len(),
            function.parameters().len(),
        ));
    }

    // Verify parameter types
    let mut annoted_parameters = Vec::new();
    let mut i = 0;
    for parameter in parameters {
        let parameter_type = parameter.get_type(output_tree, scope)?;

        if *function.parameters()[i].parameter_type() != parameter_type {
            return Err(SemanticAnalysisError::InvalidParameterType(
                name,
                i,
                parameter_type.to_string(),
                function.parameters()[i].parameter_type().to_string(),
            ));
        }

        annoted_parameters.push(parameter.semantic_analysis(output_tree, scope)?);

        i += 1;
    }

    Ok(annotated::expression::Expression::FunctionCall(
        name,
        annoted_parameters,
    ))
}
