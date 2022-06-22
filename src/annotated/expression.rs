use crate::types::Type;

pub enum Expression {
    Variable(String, Type),
    FunctionCall(String, Vec<Expression>, Type),
    FloatLiteral(f64),
    StructCreation(String, Vec<Expression>, Type),
    MemberAccess(Box<Expression>, String, Type),
    Empty,
    Multiply(Box<Expression>, Box<Expression>, Type),
}

impl Expression {
    pub fn hlsl(self) -> String {
        match self {
            Expression::Empty => String::new(),
            Expression::Variable(variable, _) => variable,
            Expression::FunctionCall(name, mut parameters, _) => {
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
            Expression::StructCreation(name, members, _) => {
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
            Expression::MemberAccess(expression, member_name, _) => {
                format!("{}.{}", expression.hlsl(), member_name)
            }
            Expression::Multiply(left_expression, right_expression, _) => {
                let left_type = left_expression.get_type();
                let right_type = right_expression.get_type();

                if (left_type.is_float_matrix()
                    && (right_type.is_float_matrix() || right_type.is_float_vector()))
                    || (right_type.is_float_matrix()
                        && (left_type.is_float_matrix() || left_type.is_float_vector()))
                {
                    format!(
                        "mul({}, {})",
                        left_expression.hlsl(),
                        right_expression.hlsl()
                    )
                } else {
                    format!("({} * {})", left_expression.hlsl(), right_expression.hlsl())
                }
            }
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Expression::Empty => Type::void(),
            Expression::Variable(_, variable_type) => variable_type.clone(),
            Expression::FunctionCall(_, _, return_type) => return_type.clone(),
            Expression::StructCreation(_, _, struct_type) => struct_type.clone(),
            Expression::FloatLiteral(_) => Type::float(),
            Expression::MemberAccess(_, _, member_type) => member_type.clone(),
            Expression::Multiply(_, _, product_type) => product_type.clone(),
        }
    }

    pub fn glsl(self) -> String {
        match self {
            Expression::Empty => String::new(),
            Expression::Variable(variable, _) => variable,
            Expression::FunctionCall(name, parameters, _) => {
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
            Expression::StructCreation(name, members, _) => {
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
            Expression::MemberAccess(expression, member_name, _) => {
                format!("{}.{}", expression.glsl(), member_name)
            }
            Expression::Multiply(left_expression, right_expression, _) => {
                format!("({} * {})", left_expression.glsl(), right_expression.glsl())
            }
        }
    }
}
