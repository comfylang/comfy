use std::collections::{HashMap, HashSet};

use chumsky::span::SimpleSpan;
use comfy_types::{Ast, Type};

pub mod access_modifier;
pub mod expression;
pub mod statements;
pub mod values;

pub trait ComfyNode<T> {
    fn to_cpp(&self, state: &mut State) -> CompileResult<T>;
    fn span(&self) -> SimpleSpan;
    fn resolve_type(&self, state: &mut State) -> CompileResult<Type>;
    fn casted_to(&self, ty: &Type, state: &mut State) -> bool {
        let sty = self.resolve_type(state);

        if let Err(e) = sty {
            state.errors.push(e);
            return false;
        } else if let Ok(sty) = sty {
            return ty == &sty; // todo: pattern matching for all types
        }

        false
    }

    fn cast_to(&self, ty: &Type, state: &mut State) -> CompileResult<Type> {
        if self.casted_to(ty, state) {
            Ok(self.resolve_type(state)?)
        } else {
            Err(Error::Compile(format!("Cannot cast",), self.span()))
        }
    }
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
    pub errors: Vec<Error>,
    pub scope_stack: Vec<HashMap<String, Ident>>,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub return_type: Type,
}

impl State {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            scope_stack: vec![HashMap::new()],
        }
    }

    pub fn set_ident(&mut self, ident: &str, type_: Type) {
        self.scope_stack
            .last_mut()
            .unwrap()
            .insert(ident.to_owned(), Ident { return_type: type_ });
    }

    pub fn get_ident_mut(&mut self, ident: &str, span: SimpleSpan) -> CompileResult<&mut Ident> {
        for scope in self.scope_stack.iter_mut().rev() {
            if let Some(ident) = scope.get_mut(ident) {
                return Ok(ident);
            }
        }

        Err(Error::Compile(
            format!("Unknown identifier: {}", ident),
            span,
        ))
    }

    pub fn get_ident(&self, ident: &str, span: SimpleSpan) -> CompileResult<&Ident> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(ident) = scope.get(ident) {
                return Ok(ident);
            }
        }

        Err(Error::Compile(
            format!("Unknown identifier: {}", ident),
            span,
        ))
    }
}

pub struct TypeInfo(bool, Option<u64>);
