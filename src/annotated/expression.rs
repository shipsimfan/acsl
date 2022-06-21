pub enum Expression {
    Variable(String),
    FunctionCall(String, Vec<Expression>),
    FloatLiteral(f64),
    StructCreation(String, Vec<Expression>),
    MemberAccess(String, String),
    Empty,
    Multiply(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn hlsl(self) -> String {
        match self {
            Expression::Empty => String::new(),
            Expression::Variable(variable) => variable,
            Expression::FunctionCall(name, mut parameters) => {
                let mut string = if name == "sample_texture" {
                    format!(
                        "{0}.Sample(acsl_{0}_sampler_state, ",
                        parameters.remove(0).hlsl()
                    )
                } else {
                    format!("{}(", name)
                };

                let mut i = 0;
                let parameters_len = parameters.len();
                for parameter in parameters {
                    string.push_str(&parameter.hlsl());

                    if i != parameters_len - 1 {
                        string.push_str(", ");
                    }

                    i += 1;
                }

                string.push(')');

                string
            }
            Expression::StructCreation(name, members) => {
                let mut hlsl = format!("acsl_create_{}(", name);

                let mut i = 0;
                let last = members.len();
                for member in members {
                    hlsl.push_str(&member.hlsl());

                    if i != last - 1 {
                        hlsl.push_str(", ");
                    }

                    i += 1;
                }

                hlsl.push(')');

                hlsl
            }
            Expression::FloatLiteral(value) => {
                if value.fract() == 0.0 {
                    format!("{}.0", value)
                } else {
                    format!("{}", value)
                }
            }
            Expression::MemberAccess(variable_name, member_name) => {
                format!("{}.{}", variable_name, member_name)
            }
            Expression::Multiply(left_expression, right_expression) => {
                format!(
                    "mul({}, {})",
                    left_expression.hlsl(),
                    right_expression.hlsl()
                )
            }
        }
    }

    pub fn glsl(self) -> String {
        match self {
            Expression::Empty => String::new(),
            Expression::Variable(variable) => variable,
            Expression::FunctionCall(name, parameters) => {
                let name = match name.as_str() {
                    "float1" => "vec1",
                    "float2" => "vec2",
                    "float3" => "vec3",
                    "float4" => "vec4",
                    "sample_texture" => "texture",
                    _ => &name,
                };

                let mut string = format!("{}(", name);

                let mut i = 0;
                let parameters_len = parameters.len();
                for parameter in parameters {
                    string.push_str(&parameter.glsl());

                    if i != parameters_len - 1 {
                        string.push_str(", ");
                    }

                    i += 1;
                }

                string.push(')');

                string
            }
            Expression::StructCreation(name, members) => {
                let mut glsl = if name == "sample_texture" {
                    format!("texture(")
                } else {
                    format!("{}(", name)
                };

                let mut i = 0;
                let last = members.len();
                for member in members {
                    glsl.push_str(&member.glsl());

                    if i != last - 1 {
                        glsl.push_str(", ");
                    }

                    i += 1;
                }

                glsl.push(')');

                glsl
            }
            Expression::FloatLiteral(value) => {
                if value.fract() == 0.0 {
                    format!("{}.0", value)
                } else {
                    format!("{}", value)
                }
            }
            Expression::MemberAccess(variable_name, member_name) => {
                format!("{}.{}", variable_name, member_name)
            }
            Expression::Multiply(left_expression, right_expression) => {
                format!("({} * {})", left_expression.glsl(), right_expression.glsl())
            }
        }
    }
}
