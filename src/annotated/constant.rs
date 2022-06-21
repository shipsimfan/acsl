use super::expression::Expression;
use crate::types::Type;

pub struct Constant {
    name: String,
    expression: Expression,
    constant_type: Type,
}

impl Constant {
    pub fn new(name: String, expression: Expression, constant_type: Type) -> Self {
        Constant {
            name,
            expression,
            constant_type,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &Type {
        &self.constant_type
    }

    pub fn generate_hlsl(self) -> String {
        format!(
            "static const {} {} = {};\n",
            self.constant_type.hlsl(),
            self.name,
            self.expression.hlsl()
        )
    }

    pub fn generate_glsl(self) -> String {
        format!(
            "const {} {} = {};\n",
            self.constant_type.glsl(),
            self.name,
            self.expression.glsl()
        )
    }
}
