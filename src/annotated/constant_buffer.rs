use crate::types::Type;

pub struct ConstantBuffer {
    name: String,
    slot: usize,
    cb_type: Type,
}

impl ConstantBuffer {
    pub fn new(name: String, slot: usize, cb_type: Type) -> Self {
        ConstantBuffer {
            name,
            slot,
            cb_type,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cb_type(&self) -> &Type {
        &self.cb_type
    }

    pub fn generate_hlsl(self) -> String {
        format!(
            "ConstantBuffer<{}> {} : register(b{});\n",
            self.cb_type.hlsl(),
            self.name,
            self.slot
        )
    }

    pub fn generate_glsl(self) -> String {
        format!(
            "layout(location = {}) uniform {} {};\n",
            self.slot,
            self.cb_type.glsl(),
            self.name
        )
    }
}
