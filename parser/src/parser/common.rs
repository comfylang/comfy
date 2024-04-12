use chumsky::{combinator::Ignored, prelude::*};
use comfy_types::{AccessModifier, Argument, Expr, Type};

use super::{expressions, types, ParseError};

pub fn ident<'a>() -> impl Parser<'a, &'a str, String, ParseError<'a>> {
    text::ident().map(ToString::to_string)
}

pub fn access_modifier<'a>() -> impl Parser<'a, &'a str, AccessModifier, ParseError<'a>> {
    choice((
        just("pub").to(AccessModifier::Public),
        just("priv").to(AccessModifier::Private),
        just("prot").to(AccessModifier::Protected),
    ))
    .padded()
}

pub fn type_descriptor<'a>() -> impl Parser<'a, &'a str, Type, ParseError<'a>> {
    justp(":")
        .ignore_then(types().padded())
        .or_not()
        .map(|t| t.unwrap_or(Type::Unknown))
        .boxed()
}

pub fn assignment<'a>() -> impl Parser<'a, &'a str, Expr, ParseError<'a>> {
    justp("=").ignore_then(expressions())
}

pub fn justp<'a>(p: &'a str) -> impl Parser<'a, &'a str, (), ParseError<'a>> {
    just(p).padded().ignored()
}

pub fn decl_args<'a>() -> impl Parser<'a, &'a str, Vec<Argument>, ParseError<'a>> {
    let arg = ident()
        .then(type_descriptor())
        .then(assignment().or_not())
        .padded()
        .map(|((name, ty), e)| Argument(name, ty, e.unwrap_or(Expr::Unknown)));

    arg.separated_by(justp(","))
        .allow_trailing()
        .collect()
        .padded()
}
