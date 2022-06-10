use super::expression::Expression;

pub enum Statement {
    Return(Expression),
}

impl Statement {
    pub fn hlsl(self) -> String {
        match self {
            Statement::Return(expression) => format!("return {};\n", expression.hlsl()),
        }
    }
}
