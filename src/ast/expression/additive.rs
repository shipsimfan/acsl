use super::{multiplicative, Expression};
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::{scope::Scope, SemanticAnalysisError},
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
};

pub fn parse(stream: &mut Stream) -> Result<(Expression, Token), ParserError> {
    let (mut left_expression, mut next_token) = multiplicative::parse(stream)?;

    loop {
        let plus = match next_token.class() {
            TokenClass::Plus => true,
            TokenClass::Dash => false,
            _ => break,
        };

        let (right_expression, nt) = multiplicative::parse(stream)?;

        left_expression = if plus {
            Expression::Add(Box::new(left_expression), Box::new(right_expression))
        } else {
            Expression::Subtract(Box::new(left_expression), Box::new(right_expression))
        };

        next_token = nt;
    }

    Ok((left_expression, next_token))
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    scope: &Scope,
    left_expression: Box<Expression>,
    right_expression: Box<Expression>,
    add: bool,
) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
    // Verify type
    let sum_type = left_expression
        .get_type(output_tree, scope)?
        .sum_type(&right_expression.get_type(output_tree, scope)?)?;

    // Get expressions
    let left_expression = left_expression.semantic_analysis(output_tree, scope)?;
    let right_expression = right_expression.semantic_analysis(output_tree, scope)?;

    Ok(if add {
        annotated::expression::Expression::Add(
            Box::new(left_expression),
            Box::new(right_expression),
            sum_type,
        )
    } else {
        annotated::expression::Expression::Subtract(
            Box::new(left_expression),
            Box::new(right_expression),
            sum_type,
        )
    })
}
