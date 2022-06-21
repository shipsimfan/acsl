use super::{scope::Scope, SemanticAnalysisError};
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    parser::ParserError,
    stream::Stream,
    tokens::Token,
    types::Type,
};

pub enum Expression {
    // Primary Expressions
    Variable(String),
    FunctionCall(String, Vec<Expression>),
    FloatLiteral(f64),
    StructCreation(String, Vec<(String, Expression)>),
    MemberAccess(Box<Expression>, String),
    Empty,

    // Multiplicative Expressions
    Multiplicative(Box<Expression>, MultiplyClass, Box<Expression>),
}

#[derive(Clone, Copy)]
pub enum MultiplyClass {
    Multiply,
}

mod multiplicative;
mod primary;

impl Expression {
    pub fn parse(stream: &mut Stream) -> Result<(Self, Token), ParserError> {
        multiplicative::parse(stream)
    }

    pub fn get_type(
        &self,
        output_tree: &AnnotatedSyntaxTree,
        scope: &Scope,
    ) -> Result<Type, SemanticAnalysisError> {
        match self {
            Expression::Empty => Ok(Type::void()),
            Expression::Variable(name) => scope
                .get_variable(name)
                .map(|variable_type| variable_type.0.clone()),
            Expression::FunctionCall(name, _) => {
                Ok(output_tree.get_function(name)?.return_type().clone())
            }
            Expression::FloatLiteral(_) => Ok(Type::float()),
            Expression::StructCreation(name, _) => output_tree.get_type(name),
            Expression::MemberAccess(expression, member) => {
                expression.get_type(output_tree, scope)?.member_type(member)
            }
            Expression::Multiplicative(left_expression, op, right_expression) => left_expression
                .get_type(output_tree, scope)?
                .multiply_type(&right_expression.get_type(output_tree, scope)?, *op),
        }
    }

    pub fn semantic_analysis(
        self,
        output_tree: &AnnotatedSyntaxTree,
        scope: &Scope,
    ) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
        match self {
            Expression::Empty => primary::empty::semantic_analysis(),
            Expression::Variable(variable) => primary::variable::semantic_analysis(scope, variable),
            Expression::FloatLiteral(value) => primary::float_literal::semantic_analysis(value),
            Expression::FunctionCall(name, parameters) => {
                primary::function_call::semantic_analysis(output_tree, scope, name, parameters)
            }
            Expression::StructCreation(name, members) => {
                primary::struct_creation::semantic_analysis(output_tree, scope, name, members)
            }
            Expression::MemberAccess(expression, member_name) => {
                primary::member_access::semantic_analysis(
                    output_tree,
                    scope,
                    expression,
                    member_name,
                )
            }
            Expression::Multiplicative(left_expression, op, right_expression) => {
                multiplicative::semantic_analysis(
                    output_tree,
                    scope,
                    left_expression,
                    op,
                    right_expression,
                )
            }
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Empty => Ok(()),
            Expression::Variable(variable) => write!(f, "{}", variable),
            Expression::FunctionCall(name, parameters) => {
                write!(f, "{}(", name)?;

                for i in 0..parameters.len() {
                    write!(f, "{}", parameters[i])?;

                    if i != parameters.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, ")")
            }
            Expression::FloatLiteral(value) => write!(f, "{}", value),
            Expression::StructCreation(name, members) => {
                write!(f, "{} {{", name)?;

                for i in 0..members.len() {
                    write!(f, "{}: {}", members[i].0, members[i].1)?;

                    if i != members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "}}")
            }
            Expression::MemberAccess(variable_name, member_name) => {
                write!(f, "{}.{}", variable_name, member_name)
            }
            Expression::Multiplicative(left_expression, op, right_expression) => {
                write!(f, "({} {} {})", left_expression, op, right_expression)
            }
        }
    }
}

impl std::fmt::Display for MultiplyClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiplyClass::Multiply => write!(f, "*"),
        }
    }
}
