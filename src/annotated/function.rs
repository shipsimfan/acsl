use super::code_block::CodeBlock;
use crate::types::Type;

pub struct Function {
    name: String,
    return_type: Type,
    parameters: Vec<FunctionParameter>,
    code_block: Option<CodeBlock>,
}

pub struct FunctionParameter {
    name: String,
    parameter_type: Type,
}

impl Function {
    pub fn new(
        name: String,
        parameters: Vec<FunctionParameter>,
        return_type: Type,
        code_block: CodeBlock,
    ) -> Self {
        Function {
            name,
            parameters,
            return_type,
            code_block: Some(code_block),
        }
    }

    pub fn builtin_functions() -> Box<[Function]> {
        vec![
            Function::new_builtin(
                "float".to_owned(),
                vec![FunctionParameter::new("x".to_owned(), Type::float())],
                Type::float(),
            ),
            Function::new_builtin(
                "float1".to_owned(),
                vec![FunctionParameter::new("x".to_owned(), Type::float())],
                Type::float1(),
            ),
            Function::new_builtin(
                "float2".to_owned(),
                vec![
                    FunctionParameter::new("x".to_owned(), Type::float()),
                    FunctionParameter::new("y".to_owned(), Type::float()),
                ],
                Type::float2(),
            ),
            Function::new_builtin(
                "float3".to_owned(),
                vec![
                    FunctionParameter::new("x".to_owned(), Type::float()),
                    FunctionParameter::new("y".to_owned(), Type::float()),
                    FunctionParameter::new("z".to_owned(), Type::float()),
                ],
                Type::float3(),
            ),
            Function::new_builtin(
                "float4".to_owned(),
                vec![
                    FunctionParameter::new("x".to_owned(), Type::float()),
                    FunctionParameter::new("y".to_owned(), Type::float()),
                    FunctionParameter::new("z".to_owned(), Type::float()),
                    FunctionParameter::new("w".to_owned(), Type::float()),
                ],
                Type::float4(),
            ),
        ]
        .into_boxed_slice()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parameters(&self) -> &[FunctionParameter] {
        &self.parameters
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }

    pub fn generate_hlsl(self) -> String {
        let mut string = format!("{} {}(", self.return_type.hlsl(), self.name);

        for i in 0..self.parameters.len() {
            string.push_str(&format!(
                "{} {}",
                self.parameters[i].parameter_type.hlsl(),
                self.parameters[i].name
            ));

            if i != self.parameters.len() - 1 {
                string.push_str(", ");
            }
        }

        string.push_str(") ");

        if &self.name == "fragment_main" {
            string.push_str(": SV_TARGET ");
        }

        string.push_str(&self.code_block.unwrap().hlsl());

        string
    }

    fn new_builtin(name: String, parameters: Vec<FunctionParameter>, return_type: Type) -> Self {
        Function {
            name,
            parameters,
            return_type,
            code_block: None,
        }
    }
}

impl FunctionParameter {
    pub fn new(name: String, parameter_type: Type) -> Self {
        FunctionParameter {
            name,
            parameter_type,
        }
    }

    pub fn parameter_type(&self) -> &Type {
        &self.parameter_type
    }
}
