use chumsky::span::SimpleSpan;
use comfy_types::{Literal, Type};

use super::{ComfyType, CompileResult, State, TypeInfo};

impl ComfyType<(String, TypeInfo)> for Type {
    fn to_c(&self, st: &mut State) -> CompileResult<(String, TypeInfo)> {
        let not_arr = TypeInfo(false, None);
        let empty_arr = TypeInfo(true, None);

        Ok(match self {
            Type::Bool(_) => ("bool".to_owned(), not_arr),
            Type::I8(_) => ("int8_t".to_owned(), not_arr),
            Type::I16(_) => ("int16_t".to_owned(), not_arr),
            Type::I32(_) => ("int32_t".to_owned(), not_arr),
            Type::I64(_) => ("int64_t".to_owned(), not_arr),
            Type::U8(_) => ("uint8_t".to_owned(), not_arr),
            Type::U16(_) => ("uint16_t".to_owned(), not_arr),
            Type::U32(_) => ("uint32_t".to_owned(), not_arr),
            Type::U64(_) => ("uint64_t".to_owned(), not_arr),
            Type::F32(_) => ("float".to_owned(), not_arr),
            Type::F64(_) => ("double".to_owned(), not_arr),
            Type::F128(_) => ("long double".to_owned(), not_arr),
            Type::Int(_) => ("int".to_owned(), not_arr),
            Type::Uint(_) => ("unsigned int".to_owned(), not_arr),
            Type::Char(_) => ("char".to_owned(), not_arr),
            Type::Str(_) => ("char".to_owned(), empty_arr),
            Type::Void(_) => ("void".to_owned(), not_arr),
            Type::Never(_) => ("void".to_owned(), not_arr),
            Type::Unknown => ("void".to_owned(), not_arr),
            Type::Tuple(_, _) => todo!(),
            Type::Array(ty, size, _) => {
                (format!("{}", ty.to_c(st)?.0), TypeInfo(true, Some(*size)))
            }
            Type::Slice(ty, _) => (format!("{}", ty.to_c(st)?.0), empty_arr),
            Type::Custom(name, _) => (name.into(), not_arr),
            Type::Pointer(ty, _) => (format!("{}*", ty.to_c(st)?.0), not_arr),
            Type::MutableRef(ty, _) => (format!("{}&", ty.to_c(st)?.0), not_arr),
            Type::Reference(ty, _) => (format!("{}&", ty.to_c(st)?.0), not_arr),
            Type::Generic(_, _, _) => todo!(),
        })
    }

    fn span(&self) -> SimpleSpan {
        match self {
            Type::Bool(s) => *s,
            Type::I8(s) => *s,
            Type::I16(s) => *s,
            Type::I32(s) => *s,
            Type::I64(s) => *s,
            Type::U8(s) => *s,
            Type::U16(s) => *s,
            Type::U32(s) => *s,
            Type::U64(s) => *s,
            Type::Int(s) => *s,
            Type::Uint(s) => *s,
            Type::F32(s) => *s,
            Type::F64(s) => *s,
            Type::F128(s) => *s,
            Type::Char(s) => *s,
            Type::Str(s) => *s,
            Type::Void(s) => *s,
            Type::Never(s) => *s,
            Type::Unknown => SimpleSpan::new(0, 0),
            Type::Tuple(_, s) => *s,
            Type::Array(_, _, s) => *s,
            Type::Slice(_, s) => *s,
            Type::Custom(_, s) => *s,
            Type::Pointer(_, s) => *s,
            Type::MutableRef(_, s) => *s,
            Type::Reference(_, s) => *s,
            Type::Generic(_, _, s) => *s,
        }
    }
}

impl ComfyType<String> for Literal {
    fn to_c(&self, _: &mut State) -> CompileResult<String> {
        Ok(match self {
            Literal::True(_) => "true".to_owned(),
            Literal::False(_) => "false".to_owned(),
            Literal::Decimal(v, _) => v.into(),
            Literal::Hex(v, _) => format!("0x{}", v),
            Literal::Octal(v, _) => format!("0{}", v),
            Literal::Binary(v, _) => format!("0b{}", v),
            Literal::Char(v, _) => format!("'{}'", v),
            Literal::Str(v, _) => format!("\"{}\"", v),
        })
    }

    fn span(&self) -> SimpleSpan {
        match self {
            Literal::True(s) => *s,
            Literal::False(s) => *s,
            Literal::Decimal(_, s) => *s,
            Literal::Hex(_, s) => *s,
            Literal::Octal(_, s) => *s,
            Literal::Binary(_, s) => *s,
            Literal::Char(_, s) => *s,
            Literal::Str(_, s) => *s,
        }
    }
}
