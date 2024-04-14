use chumsky::prelude::*;
use comfy_types::tokens::{Kind, Literal};

use super::LexError;

pub fn literals<'a>() -> impl Parser<'a, &'a str, Literal, LexError<'a>> {
    let numeric = {
        let frac = just('.').labelled("fraction");
        let pm = one_of("+-").labelled("sign");

        let exp = just('e')
            .or(just('E'))
            .then(pm.or_not())
            .labelled("exponent");

        let decimal = pm
            .or_not()
            .then(text::int(10))
            .then(frac.then(text::digits(10)).or_not())
            .then(exp.then(text::digits(10)).or_not())
            .to_slice()
            .map(|s: &str| Literal::Decimal(s.to_owned()))
            .labelled("decimal literal");

        let binary = just("0b")
            .or(just("0b"))
            .ignore_then(text::digits(2).to_slice())
            .map(|s: &str| Literal::Binary(s.to_owned()))
            .labelled("binary literal");

        let octal = just("0o")
            .or(just("0O"))
            .ignore_then(text::digits(8).to_slice())
            .map(|s: &str| Literal::Octal(s.to_owned()))
            .labelled("octal literal");

        let hex = just("0x")
            .or(just("0X"))
            .ignore_then(text::digits(16).to_slice())
            .map(|s: &str| Literal::Hex(s.to_owned()))
            .labelled("hex literal");

        choice((binary, octal, hex, decimal))
    };

    let boolean = choice((
        just("true").to(Literal::True).labelled("true"),
        just("false").to(Literal::False).labelled("false"),
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
            .map(|c| c.1)
            .labelled("escape sequence");

        let char_literal = escape
            .clone()
            .or(any().to_slice().map(ToString::to_string))
            .delimited_by(just('\''), just('\''))
            .map(Literal::Char)
            .labelled("char literal");

        let string_literal = none_of("\\\"")
            .to_slice()
            .map(ToString::to_string)
            .or(escape.clone())
            .repeated()
            .to_slice()
            .map(ToString::to_string)
            .delimited_by(just('"'), just('"'))
            .map(Literal::Str)
            .labelled("string literal");

        choice((char_literal, string_literal))
    };

    choice((textual, boolean, numeric)).labelled("literal")
}

pub fn ident<'a>() -> impl Parser<'a, &'a str, String, LexError<'a>> {
    text::ident().map(ToString::to_string)
}

pub fn token<'a>() -> impl Parser<'a, &'a str, (Kind, SimpleSpan), LexError<'a>> {
    let op1 = choice((
        just(";").to(Kind::Semicolon),
        just(",").to(Kind::Comma),
        just(":").to(Kind::Colon),
        just(".").to(Kind::Dot),
        just("+").to(Kind::Plus),
        just("-").to(Kind::Minus),
        just("*").to(Kind::Star),
        just("/").to(Kind::Slash),
        just("^").to(Kind::Caret),
        just("%").to(Kind::Percent),
        just("&").to(Kind::Ampersand),
        just("|").to(Kind::Pipe),
        just("~").to(Kind::Tilde),
        just("?").to(Kind::QuestionMark),
        just("!").to(Kind::ExclamationMark),
        just("=").to(Kind::Assign),
        just("<").to(Kind::Less),
        just(">").to(Kind::Greater),
        just("(").to(Kind::LParen),
        just(")").to(Kind::RParen),
        just("{").to(Kind::LAngle),
        just("}").to(Kind::RAngle),
        just("[").to(Kind::LSquare),
        just("]").to(Kind::RSquare),
    ));

    let op2 = choice((
        just("->").to(Kind::Arrow),
        just("<<").to(Kind::LeftShift),
        just(">>").to(Kind::RightShift),
        just("++").to(Kind::DoublePlus),
        just("--").to(Kind::DoubleMinus),
        just("+=").to(Kind::PlusAssign),
        just("-=").to(Kind::MinusAssign),
        just("*=").to(Kind::StarAssign),
        just("/=").to(Kind::SlashAssign),
        just("^=").to(Kind::CaretAssign),
        just("%=").to(Kind::PercentAssign),
        just("&=").to(Kind::AmpersandAssign),
        just("|=").to(Kind::PipeAssign),
        just("&&").to(Kind::DoubleAmp),
        just("||").to(Kind::DoublePipe),
        just("==").to(Kind::DoubleEqual),
        just("!=").to(Kind::NotEqual),
        just("<=").to(Kind::LessEqual),
        just(">=").to(Kind::GreaterEqual),
        just("as").to(Kind::As),
        just("priv").to(Kind::Priv),
        just("pub").to(Kind::Pub),
        just("prot").to(Kind::Prot),
    ));

    let ident = ident().map(|s| match s.as_str() {
        "fn" => Kind::Fn,
        "let" => Kind::Let,
        "if" => Kind::If,
        "else" => Kind::Else,
        "return" => Kind::Return,
        "while" => Kind::While,
        "for" => Kind::For,
        "in" => Kind::In,
        "break" => Kind::Break,
        "continue" => Kind::Continue,
        _ => Kind::Ident(s),
    });

    let literal = literals().map(|l| Kind::Literal(l));

    let token = op2.or(op1).or(literal).or(ident);

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|s, e| (s, e.span()))
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
}

pub fn tokens<'a>() -> impl Parser<'a, &'a str, Vec<(Kind, SimpleSpan)>, LexError<'a>> {
    token().repeated().collect()
}
