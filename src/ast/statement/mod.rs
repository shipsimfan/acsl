use super::{expression::Expression, scope::Scope, SemanticAnalysisError};
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
    types::Type,
};

mod return_statement;

pub enum Statement {
    Return(Expression),
}

impl Statement {
    pub fn parse(stream: &mut Stream, first_token: Token) -> Result<Self, ParserError> {
        match first_token.class() {
            TokenClass::Return => return_statement::parse(stream),
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
            Statement::Return(expression) => return_statement::semantic_analysis(
                output_tree,
                scope,
                function_return_type,
                expression,
            ),
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
