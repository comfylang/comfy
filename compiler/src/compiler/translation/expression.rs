use chumsky::span::SimpleSpan;
use comfy_types::{Expr, Type};
use comfy_utils::b;

use super::{ComfyNode, CompileResult, Error, State};

#[macro_export]
macro_rules! cast_format {
    ($l: ident, $op: literal, $r: ident, $st: ident, $self: ident) => {
        if $l.casted_to(&$r.resolve_type($st)?, $st) {
            format!("({} {} {})", $l.to_cpp($st)?, $op, $r.to_cpp($st)?)
        } else {
            Err(Error::Compile(
                format!("Cannot cast types, do it manually"),
                $self.span(),
            ))?
        }
    };
}

#[macro_export]
macro_rules! cast {
    ($l: ident, $r: ident, $st: ident) => {
        $l.cast_to(&$r.resolve_type($st)?, $st)
    };
}

impl ComfyNode<String> for Expr {
    fn to_cpp(&self, st: &mut State) -> CompileResult<String> {
        Ok(match self {
            Expr::Literal(l) => l.to_cpp(st)?,
            Expr::Type(t) => {
                let ct = t.to_cpp(st)?;

                if ct.1 .0 {
                    st.errors.push(Error::Compile(
                        "Cannot cast to array-like type".to_owned(),
                        self.span(),
                    ));
                }

                ct.0
            }
            Expr::Ident(i, _) => i.into(),
            Expr::Add(l, r) => cast_format!(l, "+", r, st, self),
            Expr::Sub(l, r) => cast_format!(l, "-", r, st, self),
            Expr::Mul(l, r) => cast_format!(l, "*", r, st, self),
            Expr::Div(l, r) => cast_format!(l, "/", r, st, self),
            Expr::Mod(l, r) => cast_format!(l, "%", r, st, self),
            Expr::Neg(l) => format!("(-{})", l.to_cpp(st)?),
            Expr::Pos(l) => format!("({})", l.to_cpp(st)?),
            Expr::IncR(l) => format!("({}++)", l.to_cpp(st)?),
            Expr::IncL(r) => format!("(++{})", r.to_cpp(st)?),
            Expr::DecR(l) => format!("({}--)", l.to_cpp(st)?),
            Expr::DecL(r) => format!("(--{})", r.to_cpp(st)?),
            Expr::Factorial(_) => todo!(),
            Expr::Deref(r) => format!("(*{})", r.to_cpp(st)?),
            Expr::Address(r) => format!("(&{})", r.to_cpp(st)?),
            Expr::Eq(l, r) => cast_format!(l, "==", r, st, self),
            Expr::Ne(l, r) => cast_format!(l, "!=", r, st, self),
            Expr::Lt(l, r) => cast_format!(l, "<", r, st, self),
            Expr::Le(l, r) => cast_format!(l, "<=", r, st, self),
            Expr::Gt(l, r) => cast_format!(l, ">", r, st, self),
            Expr::Ge(l, r) => cast_format!(l, ">=", r, st, self),
            Expr::And(l, r) => cast_format!(l, "&&", r, st, self),
            Expr::Or(l, r) => cast_format!(l, "||", r, st, self),
            Expr::Not(r) => format!("(!{})", r.to_cpp(st)?),
            Expr::BitAnd(l, r) => cast_format!(l, "&", r, st, self),
            Expr::BitOr(l, r) => cast_format!(l, "|", r, st, self),
            Expr::BitXor(l, r) => cast_format!(l, "^", r, st, self),
            Expr::BitNot(r) => format!("(~{})", r.to_cpp(st)?),
            Expr::Shl(l, r) => cast_format!(l, "<<", r, st, self),
            Expr::Shr(l, r) => cast_format!(l, ">>", r, st, self),
            Expr::Member(l, r) => format!("({}.{})", l.to_cpp(st)?, r.to_cpp(st)?), // TODO: check if member exists
            Expr::Cast(l, r) => format!("(static_cast<{}>({}))", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Size(r) => format!("(sizeof({}))", r.to_cpp(st)?),
            Expr::Align(r) => format!("(alignof({}))", r.to_cpp(st)?),
            Expr::Assign(l, r) => cast_format!(l, "=", r, st, self),
            Expr::AddAssign(l, r) => cast_format!(l, "+=", r, st, self),
            Expr::SubAssign(l, r) => cast_format!(l, "-=", r, st, self),
            Expr::MulAssign(l, r) => cast_format!(l, "*=", r, st, self),
            Expr::DivAssign(l, r) => cast_format!(l, "/=", r, st, self),
            Expr::ModAssign(l, r) => cast_format!(l, "%=", r, st, self),
            Expr::ShlAssign(l, r) => cast_format!(l, "<<=", r, st, self),
            Expr::ShrAssign(l, r) => cast_format!(l, ">>=", r, st, self),
            Expr::BitAndAssign(l, r) => cast_format!(l, "&=", r, st, self),
            Expr::BitXorAssign(l, r) => cast_format!(l, "^=", r, st, self),
            Expr::BitOrAssign(l, r) => cast_format!(l, "|=", r, st, self),
            Expr::Call(l, r, _) => format!("{}({})", l.to_cpp(st)?, r.to_owned().to_cpp(st)?),
            Expr::ArrMember(l, r) => format!("({}[{}])", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Tuple(_, _) => todo!(),
            Expr::Array(v, _) => {
                let arr = v
                    .iter()
                    .map(|s| s.to_cpp(st))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ");

                format!("{{ {} }}", arr)
            }
            Expr::Unknown => Err(Error::Compile("Unknown expression".to_owned(), self.span()))?,
        })
    }

    fn span(&self) -> SimpleSpan {
        match self {
            Expr::Literal(l) => l.span(),
            Expr::Type(l) => l.span(),
            Expr::Ident(_, s) => *s,
            Expr::Add(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Sub(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Mul(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Div(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Mod(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Neg(v) => v.span(),
            Expr::Pos(v) => v.span(),
            Expr::IncR(v) => v.span(),
            Expr::IncL(v) => v.span(),
            Expr::DecR(v) => v.span(),
            Expr::DecL(v) => v.span(),
            Expr::Factorial(v) => v.span(),
            Expr::Deref(v) => v.span(),
            Expr::Address(v) => v.span(),
            Expr::Eq(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Ne(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Lt(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Le(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Gt(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Ge(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::And(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Or(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Not(v) => v.span(),
            Expr::BitAnd(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::BitOr(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::BitXor(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::BitNot(v) => v.span(),
            Expr::Shl(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Shr(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Member(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Cast(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Size(v) => v.span(),
            Expr::Align(v) => v.span(),
            Expr::Assign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::AddAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::SubAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::MulAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::DivAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::ModAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::ShlAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::ShrAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::BitAndAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::BitXorAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::BitOrAssign(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Call(_, _, s) => *s,
            Expr::ArrMember(l, r) => SimpleSpan::new(l.span().start, r.span().end),
            Expr::Tuple(_, s) => *s,
            Expr::Array(_, s) => *s,
            Expr::Unknown => SimpleSpan::new(0, 0),
        }
    }

    fn resolve_type(&self, st: &mut State) -> CompileResult<Type> {
        match self {
            Expr::Literal(l) => l.resolve_type(st),
            Expr::Type(t) => t.resolve_type(st),
            Expr::Ident(name, s) => Ok(st.get_ident(name, *s)?.return_type.clone()),
            Expr::Add(l, r) => cast!(l, r, st),
            Expr::Sub(l, r) => cast!(l, r, st),
            Expr::Mul(l, r) => cast!(l, r, st),
            Expr::Div(l, r) => cast!(l, r, st),
            Expr::Mod(l, r) => cast!(l, r, st),
            Expr::Neg(r) => r.resolve_type(st),
            Expr::Pos(r) => r.resolve_type(st),
            Expr::IncR(r) => r.resolve_type(st),
            Expr::IncL(r) => r.resolve_type(st),
            Expr::DecR(r) => r.resolve_type(st),
            Expr::DecL(r) => r.resolve_type(st),
            Expr::Factorial(r) => r.resolve_type(st),
            Expr::Deref(r) => r.resolve_type(st),
            Expr::Address(r) => r.resolve_type(st),
            Expr::Eq(l, r) => cast!(l, r, st),
            Expr::Ne(l, r) => cast!(l, r, st),
            Expr::Lt(l, r) => cast!(l, r, st),
            Expr::Le(l, r) => cast!(l, r, st),
            Expr::Gt(l, r) => cast!(l, r, st),
            Expr::Ge(l, r) => cast!(l, r, st),
            Expr::And(l, r) => cast!(l, r, st),
            Expr::Or(l, r) => cast!(l, r, st),
            Expr::Not(r) => r.resolve_type(st),
            Expr::BitAnd(l, r) => cast!(l, r, st),
            Expr::BitOr(l, r) => cast!(l, r, st),
            Expr::BitXor(l, r) => cast!(l, r, st),
            Expr::BitNot(r) => r.resolve_type(st),
            Expr::Shl(l, r) => cast!(l, r, st),
            Expr::Shr(l, r) => cast!(l, r, st),
            Expr::Member(l, r) => cast!(l, r, st),
            Expr::Cast(l, r) => cast!(l, r, st),
            Expr::Size(r) => todo!(),
            Expr::Align(r) => todo!(),
            Expr::Assign(l, r) => cast!(l, r, st),
            Expr::AddAssign(l, r) => cast!(l, r, st),
            Expr::SubAssign(l, r) => cast!(l, r, st),
            Expr::MulAssign(l, r) => cast!(l, r, st),
            Expr::DivAssign(l, r) => cast!(l, r, st),
            Expr::ModAssign(l, r) => cast!(l, r, st),
            Expr::ShlAssign(l, r) => cast!(l, r, st),
            Expr::ShrAssign(l, r) => cast!(l, r, st),
            Expr::BitAndAssign(l, r) => cast!(l, r, st),
            Expr::BitXorAssign(l, r) => cast!(l, r, st),
            Expr::BitOrAssign(l, r) => cast!(l, r, st),
            Expr::Call(l, _, r) => todo!(),
            Expr::ArrMember(arr, _) => {
                let t = arr.resolve_type(st)?;

                match t {
                    Type::Array(t, _, _) => Ok(*t),

                    _ => Err(Error::Compile(
                        "Cannot get member of non array type".to_owned(),
                        self.span(),
                    )),
                }
            }
            Expr::Tuple(l, r) => todo!(),
            Expr::Array(v, s) => {
                let size = v.len();
                let typ = v.first().unwrap().resolve_type(st)?;

                let mut errored = false;
                for val in v.iter().skip(1) {
                    let t = val.resolve_type(st)?;

                    if t != typ {
                        st.errors.push(Error::Compile(
                            format!("Array types do not match, expected {:?}, got {:?}", typ, t),
                            val.span(),
                        ));

                        errored = true;
                    }
                }

                if errored {
                    st.errors
                        .push(Error::Compile("Array is not homogeneous".to_owned(), *s));
                }

                Ok(Type::Array(b(typ), size.try_into().unwrap(), *s))
            }
            Expr::Unknown => Err(Error::Compile("Unknown expression".to_owned(), self.span()))?,
        }
    }
}

impl ComfyNode<String> for Vec<Expr> {
    fn to_cpp(&self, st: &mut State) -> CompileResult<String> {
        Ok(self
            .iter()
            .map(|s| s.to_cpp(st))
            .collect::<Result<Vec<_>, _>>()?
            .join(","))
    }

    fn span(&self) -> SimpleSpan {
        let start = self.first().unwrap().span().start;
        let end = self.last().unwrap().span().end;

        SimpleSpan::new(start, end)
    }

    fn resolve_type(&self, _: &mut State) -> CompileResult<comfy_types::Type> {
        Ok(comfy_types::Type::Unknown(self.span()))
    }
}
