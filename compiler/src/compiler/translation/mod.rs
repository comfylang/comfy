use std::collections::HashSet;

use chumsky::span::SimpleSpan;
use comfy_types::{Ast, Type};

pub mod access_modifier;
pub mod expression;
pub mod statements;
pub mod values;

pub trait ComfyType<T> {
    fn to_cpp(&self, state: &mut State) -> CompileResult<T>;
    fn span(&self) -> SimpleSpan;
    fn resolve_type(&self, state: &mut State) -> CompileResult<Type>;
}

#[derive(Debug, Clone)]
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
    pub errors: Vec<Error>,
    pub scope_stack: Vec<HashSet<String>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            idents: HashSet::new(),
            errors: Vec::new(),
            scope_stack: Vec::new(),
        }
    }
}

pub struct TypeInfo(bool, Option<u64>);
