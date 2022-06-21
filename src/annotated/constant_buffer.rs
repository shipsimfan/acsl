use super::{CONSTANT_BUFFER_INDEX, MAX_CONSTANT_BUFFERS};
use crate::{ast::SemanticAnalysisError, types::Type};

#[derive(Clone)]
pub struct ConstantBuffer {
    name: String,
    slot: usize,
    cb_type: Type,
}

impl ConstantBuffer {
    pub fn new(name: String, slot: usize, cb_type: Type) -> Result<Self, SemanticAnalysisError> {
        if slot >= MAX_CONSTANT_BUFFERS {
            Err(SemanticAnalysisError::SlotOutOfRange(
                "constant buffers",
                slot,
                MAX_CONSTANT_BUFFERS,
            ))
        } else {
            Ok(ConstantBuffer {
                name,
                slot,
                cb_type,
            })
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn slot(&self) -> usize {
        self.slot
    }

    pub fn cb_type(&self) -> &Type {
        &self.cb_type
    }

    pub fn generate_hlsl(self) -> String {
        format!(
            "cbuffer acsl_constant_buffer_{} : register(b{}) {{\n    {} {};\n}}\n",
            self.slot,
            self.slot,
            self.cb_type.hlsl(),
            self.name
        )
    }

    pub fn generate_glsl(self) -> String {
        format!(
            "layout(location = {}) uniform {} {};\n",
            self.slot + CONSTANT_BUFFER_INDEX,
            self.cb_type.glsl(),
            self.name
        )
    }
}
