mod annotated;
mod ast;
mod lexer;
mod parser;
mod stream;
mod tokens;
mod types;

#[derive(Debug)]
pub enum CompilationError {
    ParserError(parser::ParserError),
    SemanticAnalysisError(ast::SemanticAnalysisError),
}

fn compile<S: AsRef<str>>(code: S) -> Result<annotated::AnnotatedSyntaxTree, CompilationError> {
    // Parse into AST
    let ast = parser::parse(code.as_ref())?;

    // Perform semantic analysis
    let annotated_ast = ast.semantic_analysis()?;

    Ok(annotated_ast)
}

pub fn compile_hlsl<S: AsRef<str>>(code: S) -> Result<String, CompilationError> {
    // Compile
    let ast = compile(code)?;

    // Generate code
    Ok(ast.generate_hlsl())
}

pub fn compile_glsl<S: AsRef<str>>(code: S) -> Result<(String, String), CompilationError> {
    // Compile
    let ast = compile(code)?;

    // Generate code
    Ok(ast.generate_glsl())
}

impl std::error::Error for CompilationError {}

impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationError::ParserError(error) => write!(f, "{}", error),
            CompilationError::SemanticAnalysisError(error) => write!(f, "{}", error),
        }
    }
}

impl From<parser::ParserError> for CompilationError {
    fn from(error: parser::ParserError) -> Self {
        CompilationError::ParserError(error)
    }
}

impl From<ast::SemanticAnalysisError> for CompilationError {
    fn from(error: ast::SemanticAnalysisError) -> Self {
        CompilationError::SemanticAnalysisError(error)
    }
}
