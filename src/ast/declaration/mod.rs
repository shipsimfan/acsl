use super::{code_block::CodeBlock, SemanticAnalysisError};
use crate::annotated::AnnotatedSyntaxTree;

pub mod function;
pub mod structure;

pub enum Declaration {
    Function(String, Vec<(String, String)>, Option<String>, CodeBlock),
    Struct(String, Vec<(String, String, Option<String>)>),
}

impl Declaration {
    pub fn semantic_analysis(
        self,
        output_tree: &mut AnnotatedSyntaxTree,
    ) -> Result<(), SemanticAnalysisError> {
        match self {
            Declaration::Function(name, parameters, return_type, code_block) => output_tree
                .push_function(function::semantic_analysis(
                    output_tree,
                    name,
                    parameters,
                    return_type,
                    code_block,
                )?),
            Declaration::Struct(name, members) => {
                output_tree.push_struct(structure::semantic_analysis(output_tree, name, members)?)
            }
        }
    }
}

impl std::fmt::Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  - ")?;

        match self {
            Declaration::Function(name, parameters, return_type, code_block) => {
                write!(f, "Function {}(", name)?;
                for i in 0..parameters.len() {
                    write!(f, "{}: {}", parameters[i].0, parameters[i].1)?;
                    if i != parameters.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")?;

                match return_type {
                    Some(return_type) => writeln!(f, " -> {}", return_type),
                    None => writeln!(f),
                }?;

                write!(f, "{}", code_block)
            }
            Declaration::Struct(name, members) => {
                writeln!(f, "Structure \"{}\"", name)?;

                for (name, type_name, semantic) in members {
                    write!(f, "      - {}: {}", name, type_name)?;
                    match semantic {
                        Some(semantic) => write!(f, " : {}", semantic)?,
                        None => {}
                    }
                    writeln!(f)?;
                }

                Ok(())
            }
        }
    }
}
