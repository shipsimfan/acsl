use super::{scope::Scope, statement::Statement, SemanticAnalysisError};
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    lexer,
    parser::ParserError,
    stream::Stream,
    tokens::TokenClass,
    types::Type,
};

pub struct CodeBlock {
    indent: usize,
    statements: Vec<Statement>,
}

impl CodeBlock {
    pub fn parse(stream: &mut Stream, indent: usize) -> Result<Self, ParserError> {
        let mut statements = Vec::new();

        loop {
            match lexer::next_token(stream)? {
                Some(token) => match token.class() {
                    TokenClass::CloseCurlyBrace => break,
                    _ => statements.push(Statement::parse(stream, token)?),
                },
                None => return Err(ParserError::UnexpectedEOF),
            };
        }

        Ok(CodeBlock { indent, statements })
    }

    pub fn semantic_analysis(
        self,
        output_tree: &AnnotatedSyntaxTree,
        scope: &mut Scope,
        function_return_type: &Type,
    ) -> Result<annotated::code_block::CodeBlock, SemanticAnalysisError> {
        let mut statements = Vec::new();

        for statement in self.statements {
            statements.push(statement.semantic_analysis(
                output_tree,
                scope,
                function_return_type,
            )?);
        }

        Ok(annotated::code_block::CodeBlock::new(
            self.indent,
            statements,
        ))
    }
}

impl std::fmt::Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            for _ in 0..self.indent * 4 {
                write!(f, " ")?;
            }

            write!(f, "  - {}", statement)?;
        }

        Ok(())
    }
}
