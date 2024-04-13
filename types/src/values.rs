use chumsky::span::SimpleSpan;

#[derive(Debug, Clone)]
pub enum Type {
    Bool(SimpleSpan),

    // Numeric
    // Signed
    I8(SimpleSpan),
    I16(SimpleSpan),
    I32(SimpleSpan),
    I64(SimpleSpan),

    // Unsigned
    U8(SimpleSpan),
    U16(SimpleSpan),
    U32(SimpleSpan),
    U64(SimpleSpan),

    // Std
    Int(SimpleSpan),
    Uint(SimpleSpan),

    // Float
    F32(SimpleSpan),
    F64(SimpleSpan),
    F128(SimpleSpan),

    // Textual
    Char(SimpleSpan),
    Str(SimpleSpan),

    // Unknown
    Void(SimpleSpan),
    Never(SimpleSpan),
    Unknown, // For parse only

    // Sequence
    Tuple(Vec<Type>, SimpleSpan),
    Array(Box<Type>, u64, SimpleSpan),
    Slice(Box<Type>, SimpleSpan),

    // User-defined
    Custom(String, SimpleSpan),
    // Struct(String),
    // Enum(String),
    // Union(String),

    // Pointers
    Pointer(Box<Type>, SimpleSpan),
    MutableRef(Box<Type>, SimpleSpan),
    Reference(Box<Type>, SimpleSpan),

    // Generic
    Generic(String, Vec<Type>, SimpleSpan),
}

#[derive(Debug, Clone)]
pub enum Literal {
    // Boolean
    True(SimpleSpan),
    False(SimpleSpan),

    // Numeric
    Decimal(String, SimpleSpan),
    Hex(String, SimpleSpan),
    Octal(String, SimpleSpan),
    Binary(String, SimpleSpan),

    // Textual
    Char(String, SimpleSpan),
    Str(String, SimpleSpan),
}
