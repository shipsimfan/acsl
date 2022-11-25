use super::{MAX_CONSTANT_BUFFERS, TEXTURES_INDEX};
use crate::{ast::SemanticAnalysisError, types::Type};

#[derive(Clone)]
pub struct Texture {
    name: String,
    slot: usize,
    texture_type: Type,
}

impl Texture {
    pub fn new(
        name: String,
        slot: usize,
        texture_type: Type,
    ) -> Result<Self, SemanticAnalysisError> {
        if slot >= MAX_CONSTANT_BUFFERS {
            Err(SemanticAnalysisError::SlotOutOfRange(
                "textures",
                slot,
                MAX_CONSTANT_BUFFERS,
            ))
        } else {
            Ok(Texture {
                name,
                slot,
                texture_type,
            })
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
            "Texture2D<{}> {} : register(t{});\nSamplerState acsl_{}_sampler_state : register(s{});\n",
            self.texture_type, self.name, self.slot, self.name, self.slot
        )
    }

    pub fn generate_glsl(self) -> String {
        format!(
            "layout(location = {}) uniform {}sampler2D {};\n",
            self.slot + TEXTURES_INDEX,
            if self.texture_type.is_uint() { "u" } else { "" },
            self.name
        )
    }
}
