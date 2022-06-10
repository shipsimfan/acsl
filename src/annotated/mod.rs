use crate::{ast::SemanticAnalysisError, types::Type};
use function::Function;
use std::{collections::VecDeque, rc::Rc};
use structure::Struct;

pub mod code_block;
pub mod expression;
pub mod function;
pub mod statement;
pub mod structure;

enum DeclarationType {
    Function,
    Struct,
}

pub struct AnnotatedSyntaxTree {
    functions: VecDeque<Function>,
    structs: VecDeque<Rc<Struct>>,

    declaration_order: Vec<DeclarationType>,

    builtin_functions: Box<[Function]>,

    vertex_input_type: Option<Type>,
    fragment_input_type: Option<Type>,
}

impl AnnotatedSyntaxTree {
    pub fn new() -> Self {
        AnnotatedSyntaxTree {
            functions: VecDeque::new(),
            structs: VecDeque::new(),
            declaration_order: Vec::new(),
            builtin_functions: Function::builtin_functions(),
            vertex_input_type: None,
            fragment_input_type: None,
        }
    }

    pub fn verify_graphics_functions(&self) -> Result<(), SemanticAnalysisError> {
        // Verify the functions have been added
        if self.vertex_input_type.is_none() {
            return Err(SemanticAnalysisError::NoVertexMain);
        } else if self.fragment_input_type.is_none() {
            return Err(SemanticAnalysisError::NoFragmentMain);
        }

        Ok(())
    }

    pub fn get_function(&self, name: &str) -> Result<&Function, SemanticAnalysisError> {
        for function in &self.functions {
            if function.name() == name {
                return Ok(function);
            }
        }

        for function in self.builtin_functions.iter() {
            if function.name() == name {
                return Ok(function);
            }
        }

        Err(SemanticAnalysisError::UnknownFunction(name.to_owned()))
    }

    pub fn get_structure(&self, name: &str) -> Result<&Struct, SemanticAnalysisError> {
        for structure in &self.structs {
            if structure.name() == name {
                return Ok(&structure);
            }
        }

        Err(SemanticAnalysisError::UnknownType(name.to_owned()))
    }

    pub fn get_type(&self, name: &str) -> Result<Type, SemanticAnalysisError> {
        for structure in &self.structs {
            if structure.name() == name {
                return Ok(Type::Struct(structure.clone()));
            }
        }

        return Err(SemanticAnalysisError::UnknownType(name.to_string()));
    }

    pub fn push_function(&mut self, function: Function) -> Result<(), SemanticAnalysisError> {
        if !self.verify_name(function.name()) {
            return Err(SemanticAnalysisError::MultipleDefinition(
                function.name().to_owned(),
            ));
        }

        if self.vertex_input_type.is_none() && function.name() == "vertex_main" {
            // Verify parameter count
            if function.parameters().len() != 1 {
                return Err(SemanticAnalysisError::VertexMainParameterCount);
            }

            // Verify input type
            let vertex_input_type = function.parameters()[0].parameter_type();
            match vertex_input_type {
                Type::Struct(_) => {} // TODO: Check for semantics
                _ => {
                    return Err(SemanticAnalysisError::InvalidVertexMainParameterType(
                        vertex_input_type.to_string(),
                    ))
                }
            }
            self.vertex_input_type = Some(vertex_input_type.clone());

            // Verify return type
            match &self.fragment_input_type {
                Some(fragement_input_type) => {
                    if function.return_type() != fragement_input_type {
                        return Err(SemanticAnalysisError::VertexMainReturnTypeMismatch(
                            function.return_type().to_string(),
                            fragement_input_type.to_string(),
                        ));
                    }
                }
                None => match function.return_type() {
                    Type::Struct(_) => {
                        // TODO: Check for semantics
                        self.fragment_input_type = Some(function.return_type().clone())
                    }
                    _ => {
                        return Err(SemanticAnalysisError::InvalidVertexMainReturnType(
                            function.return_type().to_string(),
                        ))
                    }
                },
            }
        } else if function.name() == "fragment_main" {
            // Verify parameter count
            if function.parameters().len() != 1 {
                return Err(SemanticAnalysisError::FragmentMainParameterCount);
            }

            // Verify parameter type
            match &self.fragment_input_type {
                Some(fragement_input_type) => {
                    if function.parameters()[0].parameter_type() != fragement_input_type {
                        return Err(SemanticAnalysisError::FragmentMainParameterTypeMismatch(
                            function.return_type().to_string(),
                            fragement_input_type.to_string(),
                        ));
                    }
                }
                None => match function.return_type() {
                    Type::Struct(_) => {
                        // TODO: Check for semantics
                        self.fragment_input_type = Some(function.return_type().clone())
                    }
                    _ => {
                        return Err(SemanticAnalysisError::InvalidFragmentMainParameterType(
                            function.return_type().to_string(),
                        ))
                    }
                },
            }

            // Verify return type
            if *function.return_type() != Type::float4() {
                return Err(SemanticAnalysisError::InvalidFragmentMainReturnType(
                    function.return_type().to_string(),
                ));
            }
        }

        self.declaration_order.push(DeclarationType::Function);
        self.functions.push_back(function);

        Ok(())
    }

    pub fn push_struct(&mut self, structure: Struct) -> Result<(), SemanticAnalysisError> {
        if !self.verify_name(structure.name()) {
            return Err(SemanticAnalysisError::MultipleDefinition(
                structure.name().to_owned(),
            ));
        }

        self.declaration_order.push(DeclarationType::Struct);
        self.structs.push_back(Rc::new(structure));

        Ok(())
    }

    pub fn generate_hlsl(mut self) -> String {
        let mut hlsl = format!("// Generated from Alexandria Common Shader Language\n\n");

        for declaration in self.declaration_order {
            match declaration {
                DeclarationType::Function => {
                    hlsl.push_str(&self.functions.pop_front().unwrap().generate_hlsl())
                }
                DeclarationType::Struct => {
                    hlsl.push_str(&self.structs.pop_front().unwrap().generate_hlsl())
                }
            }

            hlsl.push('\n');
        }

        hlsl
    }

    fn verify_name(&self, name: &str) -> bool {
        const BUILTIN_TYPENAMES: [&str; 5] = ["float", "float1", "float2", "float3", "float4"];

        for function in &self.functions {
            if function.name() == name {
                return false;
            }
        }

        for structure in &self.structs {
            if structure.name() == name {
                return false;
            }
        }

        for builtin_name in BUILTIN_TYPENAMES {
            if builtin_name == name {
                return false;
            }
        }

        true
    }
}
