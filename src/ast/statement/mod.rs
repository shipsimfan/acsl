use super::{expression::Expression, scope::Scope, SemanticAnalysisError};
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
    types::Type,
};

pub enum Statement {
    Return(Expression),
}

impl Statement {
    pub fn parse(stream: &mut Stream, first_token: Token) -> Result<Self, ParserError> {
        match first_token.class() {
            TokenClass::Return => {
                let (expression, next_token) = Expression::parse(stream)?;

                match next_token.class() {
                    TokenClass::SemiColon => Ok(Statement::Return(expression)),
                    _ => return Err(ParserError::UnexpectedToken(next_token)),
                }
            }
            _ => return Err(ParserError::UnexpectedToken(first_token)),
        }
    }

    pub fn semantic_analysis(
        self,
        output_tree: &AnnotatedSyntaxTree,
        scope: &mut Scope,
        function_return_type: &Type,
    ) -> Result<annotated::statement::Statement, SemanticAnalysisError> {
        match self {
            Statement::Return(expression) => {
                let expression_type = expression.get_type(output_tree, scope)?;
                if expression_type == *function_return_type {
                    Ok(annotated::statement::Statement::Return(
                        expression.semantic_analysis(output_tree, scope)?,
                    ))
                } else {
                    Err(SemanticAnalysisError::InvalidReturnType(
                        expression_type.to_string(),
                        function_return_type.to_string(),
                    ))
                }
            }
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Return(expression) => writeln!(f, "return {}", expression),
        }
    }
}
