use comfy_types::Literals;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, char, digit0, digit1},
    combinator::{map, opt, value},
    error::VerboseError,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::char;

type LiteralsReturn<'a> = IResult<&'a str, Literals, VerboseError<&'a str>>;

fn int(input: &str) -> LiteralsReturn {
    map(digit1, |s: &str| Literals::Int(s.parse().unwrap()))(input)
}

fn float(input: &str) -> LiteralsReturn {
    map(
        alt((
            tuple((digit0, char('.'), digit1)),
            tuple((digit1, char('.'), digit0)),
        )),
        |(s, _, f)| Literals::Float(format!("{}.{}", s, f).parse().unwrap()),
    )(input)
}

fn bool(input: &str) -> LiteralsReturn {
    alt((
        value(Literals::True, tag("true")),
        value(Literals::False, tag("false")),
    ))(input)
}

fn parse_char(input: &str) -> IResult<&str, char, VerboseError<&str>> {
    let (input, escaped) = opt(preceded(char('\\'), anychar))(input)?;

    if let Some(escaped) = escaped {
        let (i, r) = match escaped {
            'u' => {
                let (input, hex) = take(4usize)(input)?;
                let code = u32::from_str_radix(hex, 16).unwrap();

                (input, char::from_u32(code).unwrap())
            }

            'x' => {
                let (input, hex) = take(2usize)(input)?;
                let code = u32::from_str_radix(hex, 16).unwrap();

                (input, char::from_u32(code).unwrap())
            }

            '0' => (input, '\0'),
            'a' => (input, '\x07'),
            'b' => (input, '\x08'),
            't' => (input, '\t'),
            'n' => (input, '\n'),
            'v' => (input, '\x0B'),
            'f' => (input, '\x0C'),
            'r' => (input, '\r'),
            'e' => (input, '\x1B'),

            _ => (input, escaped),
        };

        return Ok((i, r));
    } else {
        anychar(input)
    }
}

fn parse_char_literal(input: &str) -> LiteralsReturn {
    delimited(
        char('\''),
        map(parse_char, |s| Literals::Char(s)),
        char('\''),
    )(input)
}

fn parse_string_literal(input: &str) -> LiteralsReturn {
    let res = char('\"')(input)?;

    let mut i = res.0;
    let mut r = String::new();

    loop {
        let (_, res) = opt(char('"'))(i)?;
        if !res.is_none() {
            break;
        }

        let (_, res) = opt(tag(r#"\""#))(i)?;
        if !res.is_none() {
            let res = take(2usize)(i)?;

            i = res.0;
            r.push_str(res.1);

            continue;
        }

        let res = parse_char(i)?;

        i = res.0;
        r.push(res.1);
    }

    (i, _) = char('\"')(i)?;

    Ok((i, Literals::Str(r)))
}

pub fn literals(input: &str) -> LiteralsReturn {
    alt((parse_char_literal, parse_string_literal, bool, float, int))(input)
}
