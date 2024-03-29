use super::{code_block::CodeBlock, expression::Expression, SemanticAnalysisError};
use crate::annotated::AnnotatedSyntaxTree;

pub mod constant;
pub mod constant_buffer;
pub mod function;
pub mod structure;
pub mod texture;
pub mod type_alias;

pub enum Declaration {
    Function(
        String,
        Vec<(String, String, bool)>,
        Option<String>,
        CodeBlock,
    ),
    Struct(String, Vec<(String, String, Option<String>)>),
    ConstantBuffer(String, usize, String),
    Texture(String, usize, String),
    TypeAlias(String, String),
    Constant(String, Expression),
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
            Declaration::Texture(name, slot, texture_type) => output_tree.push_texture(
                texture::semantic_analysis(name, slot, texture_type, &output_tree)?,
            ),
            Declaration::TypeAlias(name, type_name) => {
                type_alias::semantic_analysis(output_tree, name, type_name)
            }
            Declaration::Constant(name, expression) => output_tree
                .push_constant(constant::semantic_analysis(output_tree, name, expression)?),
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
            Declaration::Texture(name, slot, texture_type) => {
                writeln!(f, "Texture<{}> \"{}\" @ {}", texture_type, name, slot)
            }
            Declaration::TypeAlias(name, type_name) => {
                writeln!(f, "Type Alias {} = {}", name, type_name)
            }
            Declaration::Constant(name, expression) => {
                writeln!(f, "Constant \"{}\" = {}", name, expression)
            }
        }
    }
}
