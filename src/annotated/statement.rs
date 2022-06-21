use super::expression::Expression;
use crate::types::Type;

pub enum Statement {
    Return(Expression, Type),
    Assignment(String, Expression),
    VariableDefinition(String, Expression, Type),
}

impl Statement {
    pub fn hlsl(self) -> String {
        match self {
            Statement::Return(expression, _) => format!("return {};\n", expression.hlsl()),
            Statement::Assignment(name, expression) => {
                format!("{} = {};\n", name, expression.hlsl())
            }
            Statement::VariableDefinition(name, expression, variable_type) => {
                format!(
                    "{} {} = {};\n",
                    variable_type.hlsl(),
                    name,
                    expression.hlsl()
                )
            }
        }
    }

    pub fn glsl(
        self,
        indent: usize,
        in_vertex_main: bool,
        in_fragment_main: bool,
        position_output_name: &str,
    ) -> String {
        match self {
            Statement::Return(expression, expression_type) => {
                if in_vertex_main {
                    let mut glsl = format!(
                        "{} acsl_vertex_output = {};\n",
                        expression_type.glsl(),
                        expression.glsl(),
                    );

                    for (name, _) in expression_type.members() {
                        for _ in 0..indent * 4 {
                            glsl.push(' ');
                        }
                        glsl.push_str(&format!(
                            "acsl_pixel_input_{} = acsl_vertex_output.{};\n",
                            name, name
                        ))
                    }

                    for _ in 0..indent * 4 {
                        glsl.push(' ');
                    }

                    glsl.push_str(&format!(
                        "gl_Position = acsl_pixel_input_{};\n",
                        position_output_name
                    ));

                    for _ in 0..indent * 4 {
                        glsl.push(' ');
                    }

                    glsl.push_str("return;\n");

                    glsl
                } else if in_fragment_main {
                    let mut glsl = format!("acsl_fragment_color = {};\n", expression.glsl());
                    for _ in 0..indent * 4 {
                        glsl.push(' ');
                    }
                    glsl.push_str("return;\n");
                    glsl
                } else {
                    format!("return {};\n", expression.glsl())
                }
            }
            Statement::VariableDefinition(name, expression, variable_type) => format!(
                "{} {} = {};\n",
                variable_type.glsl(),
                name,
                expression.glsl()
            ),
            Statement::Assignment(name, expression) => {
                format!("{} = {};\n", name, expression.glsl())
            }
        }
    }
}
