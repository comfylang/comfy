use crate::Expr;

#[derive(Debug, Clone)]
pub enum Type {
    Bool,

    // Numeric
    // Signed
    I8,
    I16,
    I32,
    I64,

    // Unsigned
    U8,
    U16,
    U32,
    U64,

    // Float
    F32,
    F64,

    // Textual
    Char,
    Str,

    // Unknown
    Void,
    Never,
    Unknown, // For parse only

    // Sequence
    Tuple(Vec<Type>),
    Array(Box<Type>, u64),
    Slice(Box<Type>),

    // User-defined
    Custom(String),
    // Struct(String),
    // Enum(String),
    // Union(String),

    // Pointers
    Pointer(Box<Type>),
    MutableRef(Box<Type>),
    Reference(Box<Type>),

    // Generic
    Generic(String, Vec<Type>),
}

#[derive(Debug, Clone)]
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
