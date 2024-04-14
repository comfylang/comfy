use chumsky::prelude::*;
use comfy_types::{
    tokens::{self, Kind, TokenInput},
    AccessModifier, Argument, Expr, Type,
};

use super::{expressions, types, ParseError, TokenParseError};

pub fn ident<'a>() -> impl Parser<'a, TokenInput<'a>, String, TokenParseError<'a>> {
    select! {
        Kind::Ident(s) => s.to_owned()
    }
}

pub fn lit<'a>() -> impl Parser<'a, TokenInput<'a>, tokens::Literal, TokenParseError<'a>> {
    select! {
        Kind::Literal(s) => s
    }
}

pub fn access_modifier<'a>() -> impl Parser<'a, TokenInput<'a>, AccessModifier, TokenParseError<'a>>
{
    choice((
        just(Kind::Pub).map_with(|_, s| AccessModifier::Public(s.span())),
        just(Kind::Priv).map_with(|_, s| AccessModifier::Private(s.span())),
        just(Kind::Prot).map_with(|_, s| AccessModifier::Protected(s.span())),
    ))
    .labelled("access modifier")
}

pub fn type_descriptor<'a>() -> impl Parser<'a, TokenInput<'a>, Type, TokenParseError<'a>> {
    just(Kind::Colon)
        .ignore_then(types())
        .or_not()
        .map_with(|t, e| t.unwrap_or(Type::Unknown(e.span())))
        .labelled("type descriptor")
        .boxed()
}

pub fn fn_type_descriptor<'a>() -> impl Parser<'a, TokenInput<'a>, Type, TokenParseError<'a>> {
    just(Kind::Arrow)
        .ignore_then(types())
        .or_not()
        .map_with(|t, e| t.unwrap_or(Type::Unknown(e.span())))
        .labelled("function return type")
        .boxed()
}

pub fn assignment<'a>() -> impl Parser<'a, TokenInput<'a>, Expr, TokenParseError<'a>> {
    just(Kind::Assign)
        .ignore_then(expressions())
        .labelled("assignment")
}

pub fn justp<'a>(p: &'a str) -> impl Parser<'a, &'a str, (), ParseError<'a>> {
    just(p).padded_by(pad()).ignored()
}

pub fn pad<'a>() -> impl Parser<'a, &'a str, (), ParseError<'a>> {
    just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded()
        .repeated()
        .padded()
        .ignored()
}

pub fn decl_args<'a>() -> impl Parser<'a, TokenInput<'a>, Vec<Argument>, TokenParseError<'a>> {
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
