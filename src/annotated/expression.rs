pub enum Expression {
    Variable(String),
    FunctionCall(String, Vec<Expression>),
    FloatLiteral(f64),
    StructCreation(String, Vec<Expression>),
    MemberAccess(String, String),
    Empty,
}

impl Expression {
    pub fn hlsl(self) -> String {
        match self {
            Expression::Empty => String::new(),
            Expression::Variable(variable) => variable,
            Expression::FunctionCall(name, parameters) => {
                let mut string = format!("{}(", name);

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
                let mut hlsl = format!("{}(", name);

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
        }
    }
}