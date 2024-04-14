use chumsky::prelude::*;
use comfy_types::tokens::{Literal, Token, TokenKind};

use super::ParseError;

pub fn literals<'a>() -> impl Parser<'a, &'a str, Literal, ParseError<'a>> {
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

pub fn ident<'a>() -> impl Parser<'a, &'a str, String, ParseError<'a>> {
    text::ident().map(ToString::to_string)
}

pub fn token<'a>() -> impl Parser<'a, &'a str, Token, ParseError<'a>> {
    let op1 = choice((
        just(";").to(TokenKind::Semicolon),
        just(",").to(TokenKind::Comma),
        just(":").to(TokenKind::Colon),
        just(".").to(TokenKind::Dot),
        just("+").to(TokenKind::Plus),
        just("-").to(TokenKind::Minus),
        just("*").to(TokenKind::Star),
        just("/").to(TokenKind::Slash),
        just("^").to(TokenKind::Caret),
        just("%").to(TokenKind::Percent),
        just("&").to(TokenKind::Ampersand),
        just("|").to(TokenKind::Pipe),
        just("~").to(TokenKind::Tilde),
        just("?").to(TokenKind::QuestionMark),
        just("!").to(TokenKind::ExclamationMark),
        just("=").to(TokenKind::Assign),
        just("<").to(TokenKind::Less),
        just(">").to(TokenKind::Greater),
        just("(").to(TokenKind::LParen),
        just(")").to(TokenKind::RParen),
        just("{").to(TokenKind::LAngle),
        just("}").to(TokenKind::RAngle),
        just("[").to(TokenKind::LSquare),
        just("]").to(TokenKind::RSquare),
    ));

    let op2 = choice((
        just("->").to(TokenKind::Arrow),
        just("<<").to(TokenKind::LeftShift),
        just(">>").to(TokenKind::RightShift),
        just("++").to(TokenKind::PlusPlus),
        just("--").to(TokenKind::MinusMinus),
        just("+=").to(TokenKind::PlusAssign),
        just("-=").to(TokenKind::MinusAssign),
        just("*=").to(TokenKind::StarAssign),
        just("/=").to(TokenKind::SlashAssign),
        just("^=").to(TokenKind::CaretAssign),
        just("%=").to(TokenKind::PercentAssign),
        just("&=").to(TokenKind::AmpersandAssign),
        just("|=").to(TokenKind::PipeAssign),
        just("&&").to(TokenKind::AmpersandAmpersand),
        just("||").to(TokenKind::PipePipe),
        just("==").to(TokenKind::DoubleEqual),
        just("!=").to(TokenKind::NotEqual),
        just("<=").to(TokenKind::LessEqual),
        just(">=").to(TokenKind::GreaterEqual),
    ));

    let ident = ident().map(|s| match s.as_str() {
        "fn" => TokenKind::Fn,
        "let" => TokenKind::Let,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        "while" => TokenKind::While,
        "for" => TokenKind::For,
        "in" => TokenKind::In,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        _ => TokenKind::Identifier(s),
    });

    let literal = literals().map(|l| TokenKind::Literal(l));

    let token = op2.or(op1).or(literal).or(ident);

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|s, e| Token(s, e.span()))
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
}

pub fn tokens<'a>() -> impl Parser<'a, &'a str, Vec<Token>, ParseError<'a>> {
    token().repeated().collect()
}
