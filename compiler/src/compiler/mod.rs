use std::collections::HashSet;

use comfy_types::Ast;

pub mod access_modifier;
pub mod expression;
pub mod statements;
pub mod values;

pub trait ToC<T> {
    fn to_c(&self, compiler: &mut State) -> Result<T, CompileError>;
}

#[derive(Debug)]
pub struct CompileError(String);

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, Clone)]
pub struct Compiler {
    ast: Ast,
}

#[derive(Debug, Clone)]
pub struct State {
    pub idents: HashSet<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            idents: HashSet::new(),
        }
    }
}

pub struct TypeInfo(bool, Option<u64>);

impl Compiler {
    pub fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub fn compile(&mut self) -> CompileResult<String> {
        let mut state = State::new();
        let result = self.ast.to_c(&mut state)?;

        Ok(format!("#include <iostream>\n\n{}", result))
    }
}
