use chumsky::prelude::*;
use comfy_types::{
    tokens::{self, Kind, TokenInput},
    AccessModifier, Argument, Expr, Type,
};

use super::{expression, types, ParseError};

pub fn ident<'a>() -> impl Parser<'a, TokenInput<'a>, String, ParseError<'a>> {
    select! {
        Kind::Ident(s) => s.to_owned()
    }
}

pub fn lit<'a>() -> impl Parser<'a, TokenInput<'a>, tokens::Literal, ParseError<'a>> {
    select! {
        Kind::Literal(s) => s
    }
}

pub fn cpp_code<'a>() -> impl Parser<'a, TokenInput<'a>, String, ParseError<'a>> {
    select! {
        Kind::CppCode(s) => s
    }
}

pub fn access_modifier<'a>() -> impl Parser<'a, TokenInput<'a>, AccessModifier, ParseError<'a>> {
    choice((
        just(Kind::Pub).map_with(|_, s| AccessModifier::Public(s.span())),
        just(Kind::Priv).map_with(|_, s| AccessModifier::Private(s.span())),
        just(Kind::Prot).map_with(|_, s| AccessModifier::Protected(s.span())),
    ))
    .labelled("access modifier")
}

pub fn type_descriptor<'a>() -> impl Parser<'a, TokenInput<'a>, Type, ParseError<'a>> {
    just(Kind::Colon)
        .ignore_then(types())
        .or_not()
        .map_with(|t, e| t.unwrap_or(Type::Unknown(e.span())))
        .labelled("type descriptor")
        .boxed()
}

pub fn fn_type_descriptor<'a>() -> impl Parser<'a, TokenInput<'a>, Type, ParseError<'a>> {
    just(Kind::Arrow)
        .ignore_then(types().labelled("function return type").boxed())
        .or_not()
        .map_with(|t, e| {
            t.unwrap_or_else(|| {
                let start = e.span().end;
                Type::Unknown(SimpleSpan::new(start, start))
            })
        })
}

pub fn assignment<'a>() -> impl Parser<'a, TokenInput<'a>, Expr, ParseError<'a>> {
    just(Kind::Assign)
        .ignore_then(expression())
        .labelled("assignment")
}

pub fn decl_args<'a>() -> impl Parser<'a, TokenInput<'a>, Vec<Argument>, ParseError<'a>> {
    let arg = ident()
        .then(type_descriptor())
        .then(assignment().or_not())
        .map_with(|((name, ty), exp), e| Argument(name, ty, exp.unwrap_or(Expr::Unknown), e.span()))
        .labelled("argument");

    arg.separated_by(just(Kind::Comma))
        .allow_trailing()
        .collect()
        .labelled("arguments")
}
