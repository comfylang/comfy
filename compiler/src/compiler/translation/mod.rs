use std::collections::HashSet;

use chumsky::span::SimpleSpan;
use comfy_types::Ast;

pub mod access_modifier;
pub mod expression;
pub mod statements;
pub mod values;

pub trait ComfyType<T> {
    fn to_c(&self, compiler: &mut State) -> CompileResult<T>;
    fn span(&self) -> SimpleSpan;
}

pub enum Error {
    Compile(String, SimpleSpan),
    Clang(String),
}

pub type CompileResult<T> = Result<T, Error>;

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
