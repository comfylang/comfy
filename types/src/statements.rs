use chumsky::span::SimpleSpan;

use crate::{AccessModifier, Expr, Type};

#[derive(Debug, Clone)]
pub enum Statements {
    ExpressionStatement(Expr, SimpleSpan),
    LetStatement(String, Type, Expr, SimpleSpan),
    FunctionDeclaration(
        AccessModifier,
        String,
        Vec<Argument>,
        Type,
        Vec<Statements>,
        SimpleSpan,
    ),
    ReturnStatement(Expr, SimpleSpan),
}

#[derive(Debug, Clone)]
pub struct Argument(pub String, pub Type, pub Expr, pub SimpleSpan);
