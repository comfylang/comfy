use comfy_types::{Literal, Type};

use super::{CompileError, CompileResult, ToC, TypeInfo};

impl ToC<(String, TypeInfo)> for Type {
    fn to_c(&self) -> CompileResult<(String, TypeInfo)> {
        let not_arr = TypeInfo(false, None);
        let empty_arr = TypeInfo(true, None);

        Ok(match self {
            Type::Bool => ("bool".to_owned(), not_arr),
            Type::I8 => ("int8_t".to_owned(), not_arr),
            Type::I16 => ("int16_t".to_owned(), not_arr),
            Type::I32 => ("int32_t".to_owned(), not_arr),
            Type::I64 => ("int64_t".to_owned(), not_arr),
            Type::U8 => ("uint8_t".to_owned(), not_arr),
            Type::U16 => ("uint16_t".to_owned(), not_arr),
            Type::U32 => ("uint32_t".to_owned(), not_arr),
            Type::U64 => ("uint64_t".to_owned(), not_arr),
            Type::F32 => ("float".to_owned(), not_arr),
            Type::F64 => ("double".to_owned(), not_arr),
            Type::F128 => ("long double".to_owned(), not_arr),
            Type::Char => ("char".to_owned(), not_arr),
            Type::Str => ("char".to_owned(), empty_arr),
            Type::Void => ("void".to_owned(), not_arr),
            Type::Never => ("void".to_owned(), not_arr),
            Type::Unknown => ("void".to_owned(), not_arr),
            Type::Tuple(_) => todo!(),
            Type::Array(ty, size) => (format!("{}", ty.to_c()?.0), TypeInfo(true, Some(*size))),
            Type::Slice(ty) => (format!("{}", ty.to_c()?.0), empty_arr),
            Type::Custom(name) => (name.into(), not_arr),
            Type::Pointer(ty) => (format!("{}*", ty.to_c()?.0), not_arr),
            Type::MutableRef(ty) => (format!("{}&", ty.to_c()?.0), not_arr),
            Type::Reference(ty) => (format!("{}&", ty.to_c()?.0), not_arr),
            Type::Generic(_, _) => todo!(),
        })
    }
}

impl ToC<String> for Literal {
    fn to_c(&self) -> CompileResult<String> {
        Ok(match self {
            Literal::True => "true".to_owned(),
            Literal::False => "false".to_owned(),
            Literal::Decimal(v) => v.into(),
            Literal::Hex(v) => format!("0x{}", v),
            Literal::Octal(v) => format!("0{}", v),
            Literal::Binary(v) => format!("0b{}", v),
            Literal::Char(v) => format!("'{}'", v),
            Literal::Str(v) => format!("\"{}\"", v),
        })
    }
}
