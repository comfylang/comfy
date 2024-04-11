#[derive(Debug, Clone)]
pub enum Types {
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

    // Sequence
    Tuple(Vec<Types>),
    Array(Box<Types>, u64),
    Slice(Box<Types>),

    // User-defined
    Custom(String),
    // Struct(String),
    // Enum(String),
    // Union(String),

    // Pointers
    Pointer(Box<Types>),
    MutableRef(Box<Types>),
    Reference(Box<Types>),

    // Generic
    Generic(String, Vec<Types>),
}

#[derive(Debug, Clone)]
pub enum Literals {
    True,
    False,

    // Numeric
    Decimal(String),
    Hex(String),
    Octal(String),
    Binary(String),

    Char(String),
    Str(String),
}
