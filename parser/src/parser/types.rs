use chumsky::prelude::*;
use comfy_types::Type;

use super::{
    common::{ident, justp},
    ParseError,
};

pub fn types<'a>() -> impl Parser<'a, &'a str, Type, ParseError<'a>> {
    let bool = just("bool").to(Type::Bool);

    let numeric = choice((
        just("i8").to(Type::I8),
        just("i16").to(Type::I16),
        just("i32").to(Type::I32),
        just("i64").to(Type::I64),
        just("u8").to(Type::U8),
        just("u16").to(Type::U16),
        just("u32").to(Type::U32),
        just("u64").to(Type::U64),
        just("f32").to(Type::F32),
        just("f64").to(Type::F64),
    ));

    let textual = choice((just("char").to(Type::Char), just("str").to(Type::Str)));

    let unknown = choice((just("void").to(Type::Void), just("never").to(Type::Never)));

    let simple_types = choice((bool, numeric, textual, unknown)).boxed();
    let custom = ident().map(|s| Type::Custom(s)).boxed();

    let complex_types = recursive(|complex| {
        let t = simple_types.clone().or(complex).or(custom);

        let tuple = t
            .clone()
            .separated_by(justp(","))
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(justp("("), justp(")"))
            .map(|s| Type::Tuple(s));

        let array = t
            .clone()
            .then_ignore(justp(";"))
            .then(text::int(10).to_slice())
            .padded()
            .delimited_by(justp("["), justp("]"))
            .map(|(ty, size)| Type::Array(Box::new(ty), size.parse().unwrap()));

        let slice = t
            .clone()
            .padded()
            .delimited_by(justp("["), justp("]"))
            .map(|ty| Type::Slice(Box::new(ty)));

        let pointer = justp("*")
            .ignore_then(t.clone())
            .map(|ty| Type::Pointer(Box::new(ty)));

        let mutable = justp("&mut")
            .padded()
            .ignore_then(t.clone())
            .map(|ty| Type::MutableRef(Box::new(ty)));

        let reference = justp("&")
            .ignore_then(t.clone())
            .map(|ty| Type::Reference(Box::new(ty)));

        let generic = ident()
            .then(
                t.separated_by(justp(",").padded())
                    .allow_trailing()
                    .collect()
                    .padded()
                    .delimited_by(justp("<"), justp(">")),
            )
            .map(|(name, types)| Type::Generic(name, types));

        choice((tuple, array, slice, pointer, mutable, reference, generic)).boxed()
    });

    choice((complex_types, simple_types)).padded()
}
