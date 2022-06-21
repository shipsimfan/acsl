use super::{expression::Expression, scope::Scope, SemanticAnalysisError};
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
    types::Type,
};

mod assignment;
mod return_statement;
mod variable_definition;

pub enum Statement {
    Return(Expression),
    VariableDefinition(String, Expression, bool),
    Assignment(String, Expression),
}

impl Statement {
    pub fn parse(stream: &mut Stream, first_token: Token) -> Result<Self, ParserError> {
        match first_token.class() {
            TokenClass::Return => return_statement::parse(stream),
            TokenClass::Identifier(name) => assignment::parse(stream, name),
            TokenClass::Let => variable_definition::parse(stream),
            _ => Err(ParserError::UnexpectedToken(first_token)),
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
            Statement::Assignment(name, expression) => {
                assignment::semantic_analysis(output_tree, scope, name, expression)
            }
            Statement::VariableDefinition(name, expression, mutable) => {
                variable_definition::semantic_analysis(
                    output_tree,
                    scope,
                    name,
                    expression,
                    mutable,
                )
            }
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Return(expression) => writeln!(f, "return {}", expression),
            Statement::Assignment(name, expression) => writeln!(f, "{} = {}", name, expression),
            Statement::VariableDefinition(name, expression, mutable) => {
                writeln!(
                    f,
                    "let {}{} = {}",
                    if *mutable { "mut " } else { "" },
                    name,
                    expression
                )
            }
        }
    }
}
