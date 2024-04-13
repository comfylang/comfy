use chumsky::prelude::*;
use comfy_types::Literal;

use super::{common::pad, ParseError};

pub fn literals<'a>() -> impl Parser<'a, &'a str, Literal, ParseError<'a>> {
    let numeric = {
        let frac = just('.');
        let pm = one_of("+-");

        let exp = just('e').or(just('E')).then(pm.or_not());

        let decimal = pm
            .or_not()
            .then(text::int(10))
            .then(frac.then(text::digits(10)).or_not())
            .then(exp.then(text::digits(10)).or_not())
            .to_slice()
            .map_with(|s: &str, e| Literal::Decimal(s.to_owned(), e.span()));

        let binary = just("0b")
            .or(just("0b"))
            .ignore_then(text::digits(2).to_slice())
            .map_with(|s: &str, e| Literal::Binary(s.to_owned(), e.span()));

        let octal = just("0o")
            .or(just("0O"))
            .ignore_then(text::digits(8).to_slice())
            .map_with(|s: &str, e| Literal::Octal(s.to_owned(), e.span()));

        let hex = just("0x")
            .or(just("0X"))
            .ignore_then(text::digits(16).to_slice())
            .map_with(|s: &str, e| Literal::Hex(s.to_owned(), e.span()));

        choice((binary, octal, hex, decimal))
    };

    let boolean = choice((
        just("true").map_with(|_, e| Literal::True(e.span())),
        just("false").map_with(|_, e| Literal::False(e.span())),
    ));

    let textual = {
        let escape = just('\\')
            .ignored()
            .then(choice((
                just('\\').to(r#"\\"#.to_owned()),
                just('/').to(r#"\/"#.to_owned()),
                just('"').to(r#"\""#.to_owned()),
                just('b').to(r#"\x08"#.to_owned()),
                just('f').to(r#"\x0C"#.to_owned()),
                just('n').to(r#"\n"#.to_owned()),
                just('r').to(r#"\r"#.to_owned()),
                just('t').to(r#"\t"#.to_owned()),
                just('u')
                    .ignore_then(text::digits(16).exactly(4).to_slice())
                    .map(|s| format!("\\u{}", s)),
                just('U')
                    .ignore_then(text::digits(16).exactly(8).to_slice())
                    .map(|s| format!("\\u{}", s)),
                just('x')
                    .ignore_then(text::digits(16).exactly(2).to_slice())
                    .map(|s| format!("\\u{}", s)),
            )))
            .map(|c| c.1);

        let char_literal = escape
            .clone()
            .or(any().to_slice().map(ToString::to_string))
            .delimited_by(just('\''), just('\''))
            .map_with(|s: String, e| Literal::Char(s, e.span()));

        let string_literal = none_of("\\\"")
            .to_slice()
            .map(ToString::to_string)
            .or(escape.clone())
            .repeated()
            .to_slice()
            .map(ToString::to_string)
            .delimited_by(just('"'), just('"'))
            .map_with(|s: String, e| Literal::Str(s, e.span()));

        choice((char_literal, string_literal))
    };

    choice((textual, boolean, numeric)).padded_by(pad())
}
