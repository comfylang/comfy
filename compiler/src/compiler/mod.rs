use comfy_types::Ast;

pub mod expression;
pub mod statements;
pub mod values;

pub trait ToC<T> {
    fn to_c(&self) -> Result<T, CompileError>;
}

#[derive(Debug)]
pub struct CompileError(String);

pub type CompileResult<T> = Result<T, CompileError>;

pub struct Compiler {
    ast: Ast,
}

pub struct TypeInfo(bool, Option<u64>);

impl Compiler {
    pub fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub fn compile(&mut self) -> CompileResult<String> {
        self.ast.to_c()
    }
}
