use chumsky::span::SimpleSpan;
use comfy_types::{Literal, Type};

use super::{ComfyNode, CompileResult, Error, State, TypeInfo};

impl ComfyNode<(String, TypeInfo)> for Type {
    fn to_cpp(&self, st: &mut State) -> CompileResult<(String, TypeInfo)> {
        let not_arr = TypeInfo(false, None);
        let empty_arr = TypeInfo(true, None);

        match self {
            Type::Bool(_) => Ok(("bool".to_owned(), not_arr)),
            Type::I8(_) => Ok(("int8_t".to_owned(), not_arr)),
            Type::I16(_) => Ok(("int16_t".to_owned(), not_arr)),
            Type::I32(_) => Ok(("int32_t".to_owned(), not_arr)),
            Type::I64(_) => Ok(("int64_t".to_owned(), not_arr)),
            Type::U8(_) => Ok(("uint8_t".to_owned(), not_arr)),
            Type::U16(_) => Ok(("uint16_t".to_owned(), not_arr)),
            Type::U32(_) => Ok(("uint32_t".to_owned(), not_arr)),
            Type::U64(_) => Ok(("uint64_t".to_owned(), not_arr)),
            Type::F32(_) => Ok(("float".to_owned(), not_arr)),
            Type::F64(_) => Ok(("double".to_owned(), not_arr)),
            Type::Int(_) => Ok(("int".to_owned(), not_arr)),
            Type::Uint(_) => Ok(("unsigned int".to_owned(), not_arr)),
            Type::Char(_) => Ok(("char".to_owned(), not_arr)),
            Type::Str(_) => Ok(("char".to_owned(), empty_arr)),
            Type::Void(_) => Ok(("void".to_owned(), not_arr)),
            Type::Never(_) => Ok(("void".to_owned(), not_arr)),
            Type::Unknown(s) => Err(Error::Compile(
                "Type can't be inferred, you need to specify it".to_owned(),
                *s,
            )),
            Type::Tuple(_, _) => todo!(),
            Type::Array(ty, size, _) => {
                Ok((format!("{}", ty.to_cpp(st)?.0), TypeInfo(true, Some(*size))))
            }
            Type::Slice(ty, _) => Ok((format!("{}", ty.to_cpp(st)?.0), empty_arr)),
            Type::Custom(name, _) => Ok((name.into(), not_arr)),
            Type::Pointer(ty, _) => Ok((format!("{}*", ty.to_cpp(st)?.0), not_arr)),
            Type::MutableRef(ty, _) => Ok((format!("{}&", ty.to_cpp(st)?.0), not_arr)),
            Type::Reference(ty, _) => Ok((format!("{}&", ty.to_cpp(st)?.0), not_arr)),
            Type::Generic(_, _, _) => todo!(),
        }
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
            Type::Char(s) => *s,
            Type::Str(s) => *s,
            Type::Void(s) => *s,
            Type::Never(s) => *s,
            Type::Unknown(s) => *s,
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

    fn resolve_type(&self, _: &mut State) -> CompileResult<Type> {
        Ok(self.clone())
    }
}

impl ComfyNode<String> for Literal {
    fn to_cpp(&self, _: &mut State) -> CompileResult<String> {
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

    fn resolve_type(&self, _: &mut State) -> CompileResult<Type> {
        match self {
            Literal::True(s) => Ok(Type::Bool(*s)),
            Literal::False(s) => Ok(Type::Bool(*s)),
            Literal::Decimal(v, s) => {
                let u8 = v.parse::<u8>().is_ok();
                let u16 = v.parse::<u16>().is_ok();
                let u32 = v.parse::<u32>().is_ok();
                let u64 = v.parse::<u64>().is_ok();
                let i8 = v.parse::<i8>().is_ok();
                let i16 = v.parse::<i16>().is_ok();
                let i32 = v.parse::<i32>().is_ok();
                let i64 = v.parse::<i64>().is_ok();
                let f32 = v.parse::<f32>().is_ok();
                let f64 = v.parse::<f64>().is_ok();

                match true {
                    _ if u8 => Ok(Type::U8(*s)),
                    _ if u16 => Ok(Type::U16(*s)),
                    _ if u32 => Ok(Type::U32(*s)),
                    _ if u64 => Ok(Type::U64(*s)),
                    _ if i8 => Ok(Type::I8(*s)),
                    _ if i16 => Ok(Type::I16(*s)),
                    _ if i32 => Ok(Type::I32(*s)),
                    _ if i64 => Ok(Type::I64(*s)),
                    _ if f32 => Ok(Type::F32(*s)),
                    _ if f64 => Ok(Type::F64(*s)),
                    _ => Err(Error::Compile("Invalid number literal".to_owned(), *s)),
                }
            }
            Literal::Hex(_, _) => todo!(),
            Literal::Octal(_, _) => todo!(),
            Literal::Binary(_, _) => todo!(),
            Literal::Char(_, s) => Ok(Type::Char(*s)),
            Literal::Str(_, s) => Ok(Type::Str(*s)),
        }
    }
}
