use chumsky::prelude::*;
use comfy_types::{
    tokens::{self, Kind, TokenInput},
    Literal, Type,
};

use super::TokenParseError;

use super::{
    common::{ident, justp, pad},
    ParseError,
};

#[macro_export]
macro_rules! to {
    ($t: path) => {
        |_, e| $t(e.span())
    };
}

#[macro_export]
macro_rules! id {
    ($t: literal) => {
        Kind::Ident($t.to_owned())
    };
}

pub fn types<'a>() -> impl Parser<'a, TokenInput<'a>, Type, TokenParseError<'a>> {
    let bool = just(id!("bool")).map_with(to!(Type::Bool));

    let numeric = choice((
        just(id!("i8")).map_with(to!(Type::I8)),
        just(id!("i16")).map_with(to!(Type::I16)),
        just(id!("i32")).map_with(to!(Type::I32)),
        just(id!("i64")).map_with(to!(Type::I64)),
        just(id!("u8")).map_with(to!(Type::U8)),
        just(id!("u16")).map_with(to!(Type::U16)),
        just(id!("u32")).map_with(to!(Type::U32)),
        just(id!("u64")).map_with(to!(Type::U64)),
        just(id!("f32")).map_with(to!(Type::F32)),
        just(id!("f64")).map_with(to!(Type::F64)),
        just(id!("f128")).map_with(to!(Type::F128)),
        just(id!("int")).map_with(to!(Type::Int)),
        just(id!("uint")).map_with(to!(Type::Uint)),
    ))
    .labelled("numeric type");

    let textual = choice((
        just(id!("char")).map_with(to!(Type::Char)),
        just(id!("str")).map_with(to!(Type::Str)),
    ))
    .labelled("textual type");

    let unknown = choice((
        just(id!("void")).map_with(to!(Type::Void)),
        just(id!("never")).map_with(to!(Type::Never)),
    ))
    .labelled("void/never type");

    let simple_types = choice((bool, numeric, textual, unknown))
        .labelled("simple type")
        .boxed();

    let literal = select! {
        Kind::Literal(l) => l
    };

    let custom = ident()
        .map_with(|s, e| Type::Custom(s, e.span()))
        .labelled("user-defined type")
        .boxed();

    let complex_types = recursive(|complex| {
        let t = simple_types.clone().or(complex).or(custom);

        let tuple = t
            .clone()
            .separated_by(just(Kind::Comma))
            .allow_trailing()
            .collect()
            .delimited_by(just(Kind::LParen), just(Kind::RParen))
            .map_with(|s, e| Type::Tuple(s, e.span()))
            .labelled("tuple type");

        let array = t
            .clone()
            .then_ignore(just(Kind::Semicolon))
            .then(literal)
            .delimited_by(just(Kind::LSquare), just(Kind::RSquare))
            .try_map_with(|(ty, size), e| match size {
                tokens::Literal::Decimal(s) => match s.parse::<u64>() {
                    Ok(s) => Ok(Type::Array(Box::new(ty), s, e.span())),
                    Err(ee) => Err(Rich::custom(
                        e.span(),
                        format!("Array size must be a decimal integer literal: {}", ee),
                    )),
                },
                _ => Err(Rich::custom(
                    e.span(),
                    "Array size must be a decimal integer literal",
                )),
            })
            .labelled("array type");

        let slice = t
            .clone()
            .delimited_by(just(Kind::LSquare), just(Kind::RSquare))
            .map_with(|ty, e| Type::Slice(Box::new(ty), e.span()))
            .labelled("slice type");

        let pointer = just(Kind::Star)
            .ignore_then(t.clone())
            .map_with(|ty, e| Type::Pointer(Box::new(ty), e.span()))
            .labelled("pointer type");

        let mutable = just(Kind::Ampersand)
            .ignore_then(just(id!("mut")))
            .ignore_then(t.clone())
            .map_with(|ty, e| Type::MutableRef(Box::new(ty), e.span()))
            .labelled("mutable reference type");

        let reference = just(Kind::Ampersand)
            .ignore_then(t.clone())
            .map_with(|ty, e| Type::Reference(Box::new(ty), e.span()))
            .labelled("reference type");

        let generic = ident()
            .then(
                t.separated_by(just(Kind::Comma))
                    .allow_trailing()
                    .collect()
                    .delimited_by(just(Kind::Less), just(Kind::Greater)),
            )
            .map_with(|(name, types), e| Type::Generic(name, types, e.span()))
            .labelled("generic type");

        choice((tuple, array, slice, pointer, mutable, reference, generic))
            .labelled("complex type")
            .boxed()
    });

    choice((complex_types, simple_types)).labelled("type")
}
