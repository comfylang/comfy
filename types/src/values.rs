use chumsky::span::SimpleSpan;
use enum_procs::PartialEqVariant;

#[derive(Debug, Clone, PartialEqVariant)]
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

    // Textual
    Char(SimpleSpan),
    Str(SimpleSpan),

    // Unknown
    Void(SimpleSpan),
    Never(SimpleSpan),
    Unknown(SimpleSpan), // For parse only

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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Bool(_) => write!(f, "bool"),
            Type::I8(_) => write!(f, "i8"),
            Type::I16(_) => write!(f, "i16"),
            Type::I32(_) => write!(f, "i32"),
            Type::I64(_) => write!(f, "i64"),
            Type::U8(_) => write!(f, "u8"),
            Type::U16(_) => write!(f, "u16"),
            Type::U32(_) => write!(f, "u32"),
            Type::U64(_) => write!(f, "u64"),
            Type::Int(_) => write!(f, "int"),
            Type::Uint(_) => write!(f, "uint"),
            Type::F32(_) => write!(f, "f32"),
            Type::F64(_) => write!(f, "f64"),
            Type::Char(_) => write!(f, "char"),
            Type::Str(_) => write!(f, "str"),
            Type::Void(_) => write!(f, "void"),
            Type::Never(_) => write!(f, "never"),
            Type::Unknown(_) => write!(f, "{{unknown}}"),
            Type::Tuple(t, _) => write!(
                f,
                "({})",
                t.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Type::Array(t, size, _) => write!(f, "[{}; {}]", t, size),
            Type::Slice(t, _) => write!(f, "[{}]", t),
            Type::Custom(name, _) => write!(f, "{}", name),
            Type::Pointer(t, _) => write!(f, "*{}", t),
            Type::MutableRef(t, _) => write!(f, "&mut {}", t),
            Type::Reference(t, _) => write!(f, "&{}", t),
            Type::Generic(name, t, _) => write!(
                f,
                "{}<{}>",
                name,
                t.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
