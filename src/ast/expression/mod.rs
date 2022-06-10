use super::{scope::Scope, SemanticAnalysisError};
use crate::{
    annotated::{self, AnnotatedSyntaxTree},
    lexer,
    parser::ParserError,
    stream::Stream,
    tokens::{Token, TokenClass},
    types::Type,
};

pub enum Expression {
    Variable(String),
    FunctionCall(String, Vec<Expression>),
    FloatLiteral(f64),
    StructCreation(String, Vec<(String, Expression)>),
    MemberAccess(String, String),
    Empty,
}

impl Expression {
    pub fn parse(stream: &mut Stream) -> Result<(Self, Token), ParserError> {
        match lexer::next_token(stream)? {
            Some(token) => match token.class() {
                TokenClass::FloatLiteral(value) => match lexer::next_token(stream)? {
                    Some(next_token) => Ok((Expression::FloatLiteral(*value), next_token)),
                    None => return Err(ParserError::UnexpectedEOF),
                },
                TokenClass::Identifier(identifier) => match lexer::next_token(stream)? {
                    Some(next_token) => match next_token.class() {
                        TokenClass::OpenParenthesis => {
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
                        TokenClass::OpenCurlyBrace => {
                            let mut members = Vec::new();

                            loop {
                                let name = match lexer::next_token(stream)? {
                                    Some(token) => match token.class() {
                                        TokenClass::Identifier(identifier) => identifier.to_owned(),
                                        TokenClass::CloseCurlyBrace => break,
                                        _ => return Err(ParserError::UnexpectedToken(token)),
                                    },
                                    None => return Err(ParserError::UnexpectedEOF),
                                };

                                match lexer::next_token(stream)? {
                                    Some(token) => match token.class() {
                                        TokenClass::Colon => {}
                                        _ => return Err(ParserError::UnexpectedToken(token)),
                                    },
                                    None => return Err(ParserError::UnexpectedEOF),
                                }

                                let (expression, next_token) = Expression::parse(stream)?;

                                members.push((name, expression));

                                match next_token.class() {
                                    TokenClass::CloseCurlyBrace => break,
                                    TokenClass::Comma => {}
                                    _ => return Err(ParserError::UnexpectedToken(token)),
                                }
                            }

                            let next_token = match lexer::next_token(stream)? {
                                Some(token) => token,
                                None => return Err(ParserError::UnexpectedEOF),
                            };

                            Ok((
                                Expression::StructCreation(identifier.to_owned(), members),
                                next_token,
                            ))
                        }
                        TokenClass::Period => {
                            let member = match lexer::next_token(stream)? {
                                Some(token) => match token.class() {
                                    TokenClass::Identifier(identifier) => identifier.to_owned(),
                                    _ => return Err(ParserError::UnexpectedToken(token)),
                                },
                                None => return Err(ParserError::UnexpectedEOF),
                            };

                            let next_token = match lexer::next_token(stream)? {
                                Some(token) => token,
                                None => return Err(ParserError::UnexpectedEOF),
                            };

                            Ok((
                                Expression::MemberAccess(identifier.to_owned(), member),
                                next_token,
                            ))
                        }
                        _ => Ok((Expression::Variable(identifier.to_owned()), next_token)),
                    },
                    None => Err(ParserError::UnexpectedEOF),
                },
                _ => Ok((Expression::Empty, token)),
            },
            None => Err(ParserError::UnexpectedEOF),
        }
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
                .map(|variable_type| variable_type.clone()),
            Expression::FunctionCall(name, _) => {
                Ok(output_tree.get_function(name)?.return_type().clone())
            }
            Expression::FloatLiteral(_) => Ok(Type::float()),
            Expression::StructCreation(name, _) => output_tree.get_type(name),
            Expression::MemberAccess(variable_name, member) => {
                scope.get_variable(variable_name)?.member_type(member)
            }
        }
    }

    pub fn semantic_analysis(
        self,
        output_tree: &AnnotatedSyntaxTree,
        scope: &Scope,
    ) -> Result<annotated::expression::Expression, SemanticAnalysisError> {
        match self {
            Expression::Empty => Ok(annotated::expression::Expression::Empty),
            Expression::Variable(variable) => {
                scope.get_variable(&variable)?;
                Ok(annotated::expression::Expression::Variable(variable))
            }
            Expression::FloatLiteral(value) => {
                Ok(annotated::expression::Expression::FloatLiteral(value))
            }
            Expression::FunctionCall(name, parameters) => {
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
            Expression::StructCreation(name, mut members) => {
                let structure = output_tree.get_structure(&name)?;

                let mut s_members = Vec::new();

                for (s_name, s_type) in structure.members() {
                    // Locate defined value
                    let mut index = None;
                    for i in 0..members.len() {
                        if members[i].0 == *s_name {
                            index = Some(i);
                            break;
                        }
                    }

                    let (_, expression) = match index {
                        Some(index) => members.remove(index),
                        None => {
                            return Err(SemanticAnalysisError::MissingStructureMember(
                                name,
                                s_name.clone(),
                            ))
                        }
                    };

                    // Verify type
                    let e_type = expression.get_type(output_tree, scope)?;
                    if *s_type != e_type {
                        return Err(SemanticAnalysisError::InvalidMemberType(
                            name,
                            s_name.to_string(),
                            e_type.to_string(),
                            s_type.to_string(),
                        ));
                    }

                    // Evaluate expression
                    let expression = expression.semantic_analysis(output_tree, scope)?;

                    // Insert member
                    s_members.push(expression);
                }

                Ok(annotated::expression::Expression::StructCreation(
                    name, s_members,
                ))
            }
            Expression::MemberAccess(variable_name, member_name) => {
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
        }
    }
}
