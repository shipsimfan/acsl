use super::statement::Statement;

pub struct CodeBlock {
    indent: usize,
    statements: Vec<Statement>,
}

impl CodeBlock {
    pub fn new(indent: usize, statements: Vec<Statement>) -> Self {
        CodeBlock { indent, statements }
    }

    pub fn hlsl(self) -> String {
        let mut hlsl = format!("{{\n");

        for statement in self.statements {
            for _ in 0..self.indent * 4 {
                hlsl.push(' ');
            }

            hlsl.push_str(&statement.hlsl());
        }

        hlsl.push_str("}\n");

        hlsl
    }

    pub fn glsl(
        self,
        in_vertex_main: bool,
        in_fragment_main: bool,
        position_output_name: &str,
    ) -> String {
        let mut glsl = String::new();

        for statement in self.statements {
            for _ in 0..self.indent * 4 {
                glsl.push(' ');
            }

            glsl.push_str(&statement.glsl(
                self.indent,
                in_vertex_main,
                in_fragment_main,
                position_output_name,
            ));
        }

        glsl
    }
}
