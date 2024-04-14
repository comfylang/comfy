use chumsky::prelude::*;
use comfy_types::{
    tokens::{self, Kind, TokenInput},
    Literal,
};

use super::TokenParseError;

use super::{
    common::{lit, pad},
    ParseError,
};

pub fn literals<'a>() -> impl Parser<'a, TokenInput<'a>, Literal, TokenParseError<'a>> {
    lit()
        .map_with(|s: tokens::Literal, e| match s {
            tokens::Literal::True => Literal::True(e.span()),
            tokens::Literal::False => Literal::False(e.span()),
            tokens::Literal::Decimal(s) => Literal::Decimal(s, e.span()),
            tokens::Literal::Hex(s) => Literal::Hex(s, e.span()),
            tokens::Literal::Octal(s) => Literal::Octal(s, e.span()),
            tokens::Literal::Binary(s) => Literal::Binary(s, e.span()),
            tokens::Literal::Char(c) => Literal::Char(c, e.span()),
            tokens::Literal::Str(s) => Literal::Str(s, e.span()),
        })
        .boxed()
}
