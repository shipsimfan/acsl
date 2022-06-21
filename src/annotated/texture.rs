use crate::ast::SemanticAnalysisError;

use super::{MAX_CONSTANT_BUFFERS, TEXTURES_INDEX};

#[derive(Clone)]
pub struct Texture {
    name: String,
    slot: usize,
}

impl Texture {
    pub fn new(name: String, slot: usize) -> Result<Self, SemanticAnalysisError> {
        if slot >= MAX_CONSTANT_BUFFERS {
            Err(SemanticAnalysisError::SlotOutOfRange(
                "textures",
                slot,
                MAX_CONSTANT_BUFFERS,
            ))
        } else {
            Ok(Texture { name, slot })
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn slot(&self) -> usize {
        self.slot
    }

    pub fn generate_hlsl(self) -> String {
        format!(
            "Texture2D {} : register(t{});\nSamplerState acsl_tex_sampler_state : register(s{});\n",
            self.name, self.slot, self.slot
        )
    }

    pub fn generate_glsl(self) -> String {
        format!(
            "layout(location = {}) uniform sampler2D {};\n",
            self.slot + TEXTURES_INDEX,
            self.name
        )
    }
}
