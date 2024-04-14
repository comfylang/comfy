use chumsky::span::SimpleSpan;
pub struct Token(pub TokenKind, pub SimpleSpan);

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
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
    PlusPlus,
    /// --
    MinusMinus,
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
    AmpersandAmpersand,
    /// ||
    PipePipe,
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

    // Identifiers
    Identifier(String),

    // Literals
    Literal(Literal),
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
