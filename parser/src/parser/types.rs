use chumsky::prelude::*;
use comfy_types::Types;

use super::{common::id, ParseError};

pub fn types<'a>() -> impl Parser<'a, &'a str, Types, ParseError<'a>> {
    let bool = just("bool").to(Types::Bool);

    let numeric = choice((
        just("i8").to(Types::I8),
        just("i16").to(Types::I16),
        just("i32").to(Types::I32),
        just("i64").to(Types::I64),
        just("u8").to(Types::U8),
        just("u16").to(Types::U16),
        just("u32").to(Types::U32),
        just("u64").to(Types::U64),
        just("f32").to(Types::F32),
        just("f64").to(Types::F64),
    ));

    let textual = choice((just("char").to(Types::Char), just("str").to(Types::Str)));

    let unknown = choice((just("void").to(Types::Void), just("never").to(Types::Never)));

    let simple_types = choice((bool, numeric, textual, unknown)).boxed();
    let custom = id().map(|s| Types::Custom(s)).boxed();

    let complex_types = recursive(|complex| {
        let t = simple_types.clone().or(complex).or(custom);

        let tuple = t
            .clone()
            .separated_by(just(',').padded())
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(just('('), just(')'))
            .map(|s| Types::Tuple(s));

        let array = t
            .clone()
            .then(just(";").padded().ignored())
            .then(text::int(10))
            .padded()
            .delimited_by(just('['), just(']'))
            .map(|((ty, _), size): (_, &str)| Types::Array(Box::new(ty), size.parse().unwrap()));

        let slice = t
            .clone()
            .padded()
            .delimited_by(just('['), just(']'))
            .map(|ty| Types::Slice(Box::new(ty)));

        let pointer = just('*')
            .then(t.clone())
            .map(|(_, ty)| Types::Pointer(Box::new(ty)));

        let mutable = just("&mut")
            .padded()
            .then(t.clone())
            .map(|(_, ty)| Types::MutableRef(Box::new(ty)));

        let reference = just('&')
            .then(t.clone())
            .map(|(_, ty)| Types::Reference(Box::new(ty)));

        let generic = id()
            .then(
                t.separated_by(just(',').padded())
                    .allow_trailing()
                    .collect()
                    .padded()
                    .delimited_by(just('<'), just('>')),
            )
            .map(|(name, types)| Types::Generic(name, types));

        choice((tuple, array, slice, pointer, mutable, reference, generic)).boxed()
    });

    choice((complex_types, simple_types)).padded()
}
