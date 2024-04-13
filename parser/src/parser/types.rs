use chumsky::prelude::*;
use comfy_types::Type;

use super::{
    common::{ident, justp},
    ParseError,
};

#[macro_export]
macro_rules! to {
    ($t: path) => {
        |_, e| $t(e.span())
    };
}

pub fn types<'a>() -> impl Parser<'a, &'a str, Type, ParseError<'a>> {
    let bool = just("bool").map_with(to!(Type::Bool));

    let numeric = choice((
        just("i8").map_with(to!(Type::I8)),
        just("i16").map_with(to!(Type::I16)),
        just("i32").map_with(to!(Type::I32)),
        just("i64").map_with(to!(Type::I64)),
        just("u8").map_with(to!(Type::U8)),
        just("u16").map_with(to!(Type::U16)),
        just("u32").map_with(to!(Type::U32)),
        just("u64").map_with(to!(Type::U64)),
        just("f32").map_with(to!(Type::F32)),
        just("f64").map_with(to!(Type::F64)),
        just("f128").map_with(to!(Type::F128)),
        just("int").map_with(to!(Type::Int)),
        just("uint").map_with(to!(Type::Uint)),
    ));

    let textual = choice((
        just("char").map_with(to!(Type::Char)),
        just("str").map_with(to!(Type::Str)),
    ));

    let unknown = choice((
        just("void").map_with(to!(Type::Void)),
        just("never").map_with(to!(Type::Never)),
    ));

    let simple_types = choice((bool, numeric, textual, unknown)).boxed();
    let custom = ident().map_with(|s, e| Type::Custom(s, e.span())).boxed();

    let complex_types = recursive(|complex| {
        let t = simple_types.clone().or(complex).or(custom);

        let tuple = t
            .clone()
            .separated_by(justp(","))
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(justp("("), justp(")"))
            .map_with(|s, e| Type::Tuple(s, e.span()));

        let array = t
            .clone()
            .then_ignore(justp(";"))
            .then(text::int(10).to_slice())
            .padded()
            .delimited_by(justp("["), justp("]"))
            .map_with(|(ty, size), e| Type::Array(Box::new(ty), size.parse().unwrap(), e.span()));

        let slice = t
            .clone()
            .padded()
            .delimited_by(justp("["), justp("]"))
            .map_with(|ty, e| Type::Slice(Box::new(ty), e.span()));

        let pointer = justp("*")
            .ignore_then(t.clone())
            .map_with(|ty, e| Type::Pointer(Box::new(ty), e.span()));

        let mutable = justp("&mut")
            .padded()
            .ignore_then(t.clone())
            .map_with(|ty, e| Type::MutableRef(Box::new(ty), e.span()));

        let reference = justp("&")
            .ignore_then(t.clone())
            .map_with(|ty, e| Type::Reference(Box::new(ty), e.span()));

        let generic = ident()
            .then(
                t.separated_by(justp(",").padded())
                    .allow_trailing()
                    .collect()
                    .padded()
                    .delimited_by(justp("<"), justp(">")),
            )
            .map_with(|(name, types), e| Type::Generic(name, types, e.span()));

        choice((tuple, array, slice, pointer, mutable, reference, generic)).boxed()
    });

    choice((complex_types, simple_types)).padded()
}
