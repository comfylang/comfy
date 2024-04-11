use chumsky::prelude::*;

use super::ParseError;

pub fn ident<'a>() -> impl Parser<'a, &'a str, String, ParseError<'a>> {
    text::ident().map(ToString::to_string)
}
