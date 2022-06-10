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
}
