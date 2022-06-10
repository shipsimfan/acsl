use self::declaration::Declaration;
use crate::annotated::AnnotatedSyntaxTree;

pub mod code_block;
pub mod declaration;
pub mod expression;
pub mod scope;
pub mod statement;

#[derive(Debug)]
pub enum SemanticAnalysisError {
    MultipleDefinition(String),
    NoVertexMain,
    NoFragmentMain,
    UnknownType(String),
    UnknownVariable(String),
    UnknownFunction(String),
    UnknownStructure(String),
    InvalidParameterCount(String, usize, usize),
    InvalidParameterType(String, usize, String, String),
    InvalidReturnType(String, String),
    InvalidMemberType(String, String, String, String),
    MissingStructureMember(String, String),
    InvalidMember(String, String),
    VertexMainParameterCount,
    InvalidVertexMainParameterType(String),
    InvalidVertexMainReturnType(String),
    VertexMainReturnTypeMismatch(String, String),
    FragmentMainParameterCount,
    InvalidFragmentMainParameterType(String),
    InvalidFragmentMainReturnType(String),
    FragmentMainParameterTypeMismatch(String, String),
    AllFieldsNeedSemantics(String),
}

pub struct AbstractSyntaxTree {
    declarations: Vec<Declaration>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        AbstractSyntaxTree {
            declarations: Vec::new(),
        }
    }

    pub fn push(&mut self, declaration: Declaration) {
        self.declarations.push(declaration)
    }

    pub fn semantic_analysis(self) -> Result<AnnotatedSyntaxTree, SemanticAnalysisError> {
        let mut output_tree = AnnotatedSyntaxTree::new();

        for declaration in self.declarations {
            declaration.semantic_analysis(&mut output_tree)?;
        }

        output_tree.verify_graphics_functions()?;

        Ok(output_tree)
    }
}

impl std::fmt::Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Abstract Syntax Tree:")?;

        for declaration in &self.declarations {
            write!(f, "{}", declaration)?;
        }

        Ok(())
    }
}

impl std::error::Error for SemanticAnalysisError {}

impl std::fmt::Display for SemanticAnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticAnalysisError::MultipleDefinition(identifier) => {
                write!(f, "Multiple definitions of \"{}\"", identifier)
            }
            SemanticAnalysisError::NoVertexMain => {
                write!(f, "No \"vertex_main()\" function defined")
            }
            SemanticAnalysisError::NoFragmentMain => {
                write!(f, "No \"fragment_main()\" function defined")
            }
            SemanticAnalysisError::UnknownType(type_name) => {
                write!(f, "Unknown type name \"{}\"", type_name)
            }
            SemanticAnalysisError::UnknownVariable(name) => {
                write!(f, "Unknown variable name \"{}\"", name)
            }
            SemanticAnalysisError::UnknownFunction(name) => {
                write!(f, "Unknown function \"{}\"", name)
            }
            SemanticAnalysisError::UnknownStructure(name) => write!(f, "Unknown structure \"{}\"", name),
            SemanticAnalysisError::InvalidParameterCount(name, actual, expected) => write!(
                f,
                "Function \"{}\" expects {} parameters, only {} provided",
                name, expected, actual
            ),
            SemanticAnalysisError::InvalidParameterType(name, index, actual, expected) => write!(f, "Function \"{}\" expects parameter {} to be of type \"{}\", instead found type \"{}\"", name, *index + 1, expected, actual),
            SemanticAnalysisError::InvalidReturnType(actual, expected) => write!(f, "Invalid return type \"{}\", expected \"{}\"", actual, expected),
            SemanticAnalysisError::MissingStructureMember(structure_name, member_name) => write!(f, "Missing member \"{}\" from initialization of struct \"{}\"", member_name, structure_name),
            SemanticAnalysisError::InvalidMemberType(structure_name, member_name, actual, expected) => write!(f, "\"{}\" in struct \"{}\" has type \"{}\", found \"{}\" instead", member_name, structure_name, expected, actual),
            SemanticAnalysisError::InvalidMember(structure_name, member_name) => write!(f, "Struct \"{}\" does not contain a member named \"{}\"", structure_name, member_name),
            SemanticAnalysisError::VertexMainParameterCount => write!(f, "\"vertex_main()\" must take one parameter"),
            SemanticAnalysisError::InvalidVertexMainParameterType(actual) => write!(f, "The parameter for \"vertex_main()\" must be a structure with semantics, instead it is \"{}\"", actual),
            SemanticAnalysisError::InvalidVertexMainReturnType(actual) => write!(f, "The return type for \"vertex_main()\" must be a structure with semantics, instead is it \"{}\"", actual),
            SemanticAnalysisError::VertexMainReturnTypeMismatch(actual, expected) => write!(f, "The return type of \"vertex_main()\" must be the same as the parameter type \"fragment_main()\" (\"{}\"), instead it is \"{}\"", expected, actual),
            SemanticAnalysisError::FragmentMainParameterCount => write!(f, "\"fragment_main()\" must take one parameter"),
            SemanticAnalysisError::FragmentMainParameterTypeMismatch(actual, expected) => write!(f, "The parameter for \"fragment_main()\" must be the same as the return type of \"vertex_main()\" (\"{}\"), instead it is \"{}\"", expected, actual),
            SemanticAnalysisError::InvalidFragmentMainReturnType(actual) => write!(f, "\"fragment_main()\" must return a float4, instead it is \"{}\"", actual),
            SemanticAnalysisError::InvalidFragmentMainParameterType(actual) => write!(f, "The parameter for \"fragment_main()\" must be a structure with semantics, instead it is \"{}\"", actual),
            SemanticAnalysisError::AllFieldsNeedSemantics(structure_name) => write!(f, "All fields require semantics when they are defined, and \"{}\" is missing some", structure_name),
        }
    }
}
