use super::{code_block::CodeBlock, SemanticAnalysisError};
use crate::annotated::AnnotatedSyntaxTree;

pub mod constant_buffer;
pub mod function;
pub mod structure;
pub mod texture;

pub enum Declaration {
    Function(String, Vec<(String, String)>, Option<String>, CodeBlock),
    Struct(String, Vec<(String, String, Option<String>)>),
    ConstantBuffer(String, usize, String),
    Texture(String, usize),
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
            Declaration::ConstantBuffer(name, slot, type_name) => output_tree.push_constant_buffer(
                constant_buffer::semantic_analysis(output_tree, name, slot, type_name)?,
            ),
            Declaration::Texture(name, slot) => {
                output_tree.push_texture(texture::semantic_analysis(name, slot)?)
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
            Declaration::ConstantBuffer(name, slot, type_name) => {
                writeln!(f, "Constant Buffer \"{}\" @ {} ({})", name, slot, type_name)
            }
            Declaration::Texture(name, slot) => writeln!(f, "Texture \"{}\" @ {}", name, slot),
        }
    }
}
