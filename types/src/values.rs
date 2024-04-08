#[derive(Debug)]
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
    Tuple,
    Array,
    Slice,

    // User-defined
    Struct(String),
    Enum(String),
    Union(String),

    // Pointer
    Pointer(Box<Types>),
    Reference(Box<Types>),
    Mutable(Box<Types>),
}

#[derive(Debug, Clone)]
pub enum Literals {
    True,
    False,
    Int(i32),
    Float(f32),
    Char(char),
    Str(String),
}
