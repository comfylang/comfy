use core::fmt;

use chumsky::{input::SpannedInput, span::SimpleSpan};

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    /// ;
    Semicolon,
    /// ,
    Comma,
    /// :
    Colon,
    /// ->
    Arrow,
    /// .
    Dot,

    // Operators
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// ^
    Caret,
    /// %
    Percent,

    /// &
    Ampersand,
    /// |
    Pipe,
    /// ~
    Tilde,
    /// <<        
    LeftShift,
    /// >>
    RightShift,
    /// ++
    DoublePlus,
    /// --
    DoubleMinus,
    /// ?
    QuestionMark,
    /// !
    ExclamationMark,

    /// =
    Assign,
    /// +=
    PlusAssign,
    /// -=
    MinusAssign,
    /// *=
    StarAssign,
    /// /=
    SlashAssign,
    /// ^=
    CaretAssign,
    /// %=
    PercentAssign,
    /// &=
    AmpersandAssign,
    /// |=
    PipeAssign,
    /// <<=
    LeftShiftAssign,
    /// >>=
    RightShiftAssign,

    // logic operators
    /// &&
    DoubleAmp,
    /// ||
    DoublePipe,
    /// ==          
    DoubleEqual,
    /// !=              
    NotEqual,
    /// <         
    Less,
    /// <=             
    LessEqual,
    /// >         
    Greater,
    /// >=          
    GreaterEqual,

    // Brackets
    /// (
    LParen,
    /// )  
    RParen,
    /// {
    LAngle,
    /// }
    RAngle,
    /// [
    LSquare,
    /// ]
    RSquare,

    // Keywords
    Fn,
    Let,
    If,
    Else,
    Return,
    While,
    For,
    In,
    Break,
    Continue,
    As,
    Sizeof,
    Alignof,
    Pub,
    Priv,
    Prot,

    // Identifiers
    Ident(String),

    // Literals
    Literal(Literal),
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Ident(s) => write!(f, "identifier '{}'", s),
            Kind::Literal(l) => write!(f, "literal {}", l),
            Kind::Semicolon => write!(f, ";"),
            Kind::Comma => write!(f, ","),
            Kind::Colon => write!(f, ":"),
            Kind::Arrow => write!(f, "->"),
            Kind::Dot => write!(f, "."),
            Kind::Plus => write!(f, "+"),
            Kind::Minus => write!(f, "-"),
            Kind::Star => write!(f, "*"),
            Kind::Slash => write!(f, "/"),
            Kind::Caret => write!(f, "^"),
            Kind::Percent => write!(f, "%"),
            Kind::Ampersand => write!(f, "&"),
            Kind::Pipe => write!(f, "|"),
            Kind::Tilde => write!(f, "~"),
            Kind::LeftShift => write!(f, "<<"),
            Kind::RightShift => write!(f, ">>"),
            Kind::DoublePlus => write!(f, "++"),
            Kind::DoubleMinus => write!(f, "--"),
            Kind::QuestionMark => write!(f, "?"),
            Kind::ExclamationMark => write!(f, "!"),
            Kind::Assign => write!(f, "="),
            Kind::PlusAssign => write!(f, "+="),
            Kind::MinusAssign => write!(f, "-="),
            Kind::StarAssign => write!(f, "*="),
            Kind::SlashAssign => write!(f, "/="),
            Kind::CaretAssign => write!(f, "^="),
            Kind::PercentAssign => write!(f, "%="),
            Kind::AmpersandAssign => write!(f, "&="),
            Kind::PipeAssign => write!(f, "|="),
            Kind::LeftShiftAssign => write!(f, "<<="),
            Kind::RightShiftAssign => write!(f, ">>="),
            Kind::DoubleAmp => write!(f, "&&"),
            Kind::DoublePipe => write!(f, "||"),
            Kind::DoubleEqual => write!(f, "=="),
            Kind::NotEqual => write!(f, "!="),
            Kind::Less => write!(f, "<"),
            Kind::LessEqual => write!(f, "<="),
            Kind::Greater => write!(f, ">"),
            Kind::GreaterEqual => write!(f, ">="),
            Kind::LParen => write!(f, "("),
            Kind::RParen => write!(f, ")"),
            Kind::LAngle => write!(f, "{{"),
            Kind::RAngle => write!(f, "}}"),
            Kind::LSquare => write!(f, "["),
            Kind::RSquare => write!(f, "]"),
            Kind::Fn => write!(f, "fn"),
            Kind::Let => write!(f, "let"),
            Kind::If => write!(f, "if"),
            Kind::Else => write!(f, "else"),
            Kind::Return => write!(f, "return"),
            Kind::While => write!(f, "while"),
            Kind::For => write!(f, "for"),
            Kind::In => write!(f, "in"),
            Kind::Break => write!(f, "break"),
            Kind::Continue => write!(f, "continue"),
            Kind::As => write!(f, "as"),
            Kind::Sizeof => write!(f, "sizeof"),
            Kind::Alignof => write!(f, "alignof"),
            Kind::Pub => write!(f, "pub"),
            Kind::Priv => write!(f, "priv"),
            Kind::Prot => write!(f, "prot"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    // Boolean
    True,
    False,

    // Numeric
    Decimal(String),
    Hex(String),
    Octal(String),
    Binary(String),

    // Textual
    Char(String),
    Str(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Char(s) => write!(f, "character '{}'", s),
            Literal::Str(s) => write!(f, "string '{}'", s),
            Literal::Decimal(s) => write!(f, "{}", s),
            Literal::Hex(s) => write!(f, "0x{}", s),
            Literal::Octal(s) => write!(f, "0o{}", s),
            Literal::Binary(s) => write!(f, "0b{}", s),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
        }
    }
}

pub type TokenInput<'a> = SpannedInput<Kind, SimpleSpan, &'a [(Kind, SimpleSpan)]>;
