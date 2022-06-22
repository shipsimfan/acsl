use super::{primary, Expression, MultiplyClass};
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    ast::{scope::Scope, SemanticAnalysisError},
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
};

pub fn parse(stream: &mut Stream) -> Result<(Expression, Token), ParserError> {
    let (mut left_expression, mut next_token) = primary::parse(stream)?;

    loop {
        match next_token.class() {
            TokenClass::Asterick => {
                let (right_expression, nt) = primary::parse(stream)?;

                left_expression = Expression::Multiplicative(
                    Box::new(left_expression),
                    MultiplyClass::Multiply,
                    Box::new(right_expression),
                );
                next_token = nt;
            }
            _ => break,
        }
    }

    Ok((left_expression, next_token))
}

pub fn semantic_analysis(
    output_tree: &AnnotatedSyntaxTree,
    scope: &Scope,
    left_expression: Box<Expression>,
    op: MultiplyClass,
    right_expression: Box<Expression>,
) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
    // Verify type
    let product_type = left_expression
        .get_type(output_tree, scope)?
        .product_type(&right_expression.get_type(output_tree, scope)?, op)?;

    // Get expressions
    let left_expression = left_expression.semantic_analysis(output_tree, scope)?;
    let right_expression = right_expression.semantic_analysis(output_tree, scope)?;

    Ok(match op {
        MultiplyClass::Multiply => annotated::expression::Expression::Multiply(
            Box::new(left_expression),
            Box::new(right_expression),
            product_type,
        ),
    })
}
