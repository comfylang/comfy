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
pub struct CompileError(pub String);

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, Clone)]
pub struct Compiler {
    pub ast: Ast,
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
