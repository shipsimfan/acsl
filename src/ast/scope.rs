use crate::types::Type;
use std::collections::HashMap;

use super::SemanticAnalysisError;

pub struct Scope {
    variables: HashMap<String, (Type, bool)>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }

    pub fn new_child(&self) -> Self {
        Scope {
            variables: self.variables.clone(),
        }
    }

    pub fn get_variable(&self, name: &str) -> Result<&(Type, bool), SemanticAnalysisError> {
        match self.variables.get(name) {
            Some(variable) => Ok(variable),
            None => Err(SemanticAnalysisError::UnknownVariable(name.to_owned())),
        }
    }

    pub fn define_variable(
        &mut self,
        name: String,
        variable_type: Type,
        mutable: bool,
    ) -> Result<(), SemanticAnalysisError> {
        // Builtin names for ACSL and GLSL
        if name.starts_with("acsl_") || name.starts_with("gl_") {
            return Err(SemanticAnalysisError::InvalidVariableName(name));
        }

        match self
            .variables
            .insert(name.clone(), (variable_type, mutable))
        {
            Some(_) => Err(SemanticAnalysisError::MultipleDefinition(name)),
            None => Ok(()),
        }
    }
}
