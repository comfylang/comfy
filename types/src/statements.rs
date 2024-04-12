use crate::{AccessModifier, Expr, Type};

#[derive(Debug, Clone)]
pub enum Statements {
    ExpressionStatement(Expr),
    LetStatement(String, Type, Expr),
    FunctionDeclaration(AccessModifier, String, Vec<Argument>, Type, Vec<Statements>),
    ReturnStatement(Expr),
}

#[derive(Debug, Clone)]
pub struct Argument(pub String, pub Type, pub Expr);
