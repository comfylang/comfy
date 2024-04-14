use chumsky::span::SimpleSpan;
use comfy_types::{Expr, Type};
use comfy_utils::{b, inc_indent};

use super::{ComfyType, CompileResult, Error, State};

impl ComfyType<String> for Expr {
    fn to_cpp(&self, st: &mut State) -> CompileResult<String> {
        Ok(match self {
            Expr::Literal(l) => l.to_cpp(st)?,
            Expr::Type(t) => {
                let ct = t.to_cpp(st)?;
                let span = self.span();

                if ct.1 .0 {
                    st.errors.push(Error::Compile(
                        "Cannot cast to array-like type".to_owned(),
                        span,
                    ));
                }

                ct.0
            }
            Expr::Ident(i, _) => i.into(),
            Expr::Add(l, r) => format!("({} + {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Sub(l, r) => format!("({} - {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Mul(l, r) => format!("({} * {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Div(l, r) => format!("({} / {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Mod(l, r) => format!("({} % {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Neg(l) => format!("-({})", l.to_cpp(st)?),
            Expr::Pos(l) => format!("({})", l.to_cpp(st)?),
            Expr::IncR(l) => format!("({}++)", l.to_cpp(st)?),
            Expr::IncL(r) => format!("(++{})", r.to_cpp(st)?),
            Expr::DecR(l) => format!("({}--)", l.to_cpp(st)?),
            Expr::DecL(r) => format!("(--{})", r.to_cpp(st)?),
            Expr::Factorial(_) => todo!(),
            Expr::Deref(r) => format!("(*{})", r.to_cpp(st)?),
            Expr::Address(r) => format!("(&{})", r.to_cpp(st)?),
            Expr::Eq(l, r) => format!("({} == {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Ne(l, r) => format!("({} != {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Lt(l, r) => format!("({} < {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Le(l, r) => format!("({} <= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Gt(l, r) => format!("({} > {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Ge(l, r) => format!("({} >= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::And(l, r) => format!("({} && {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Or(l, r) => format!("({} || {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Not(r) => format!("(!{})", r.to_cpp(st)?),
            Expr::BitAnd(l, r) => format!("({} & {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::BitOr(l, r) => format!("({} | {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::BitXor(l, r) => format!("({} ^ {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::BitNot(r) => format!("(~{})", r.to_cpp(st)?),
            Expr::Shl(l, r) => format!("({} << {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Shr(l, r) => format!("({} >> {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Member(l, r) => format!("({}.{})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::Cast(l, r) => format!("(({}) {} )", r.to_cpp(st)?, l.to_cpp(st)?),
            Expr::Size(r) => format!("sizeof ({})", r.to_cpp(st)?),
            Expr::Align(r) => format!("alignof ({})", r.to_cpp(st)?),
            Expr::Assign(l, r) => format!("({} = {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::AddAssign(l, r) => format!("({} += {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::SubAssign(l, r) => format!("({} -= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::MulAssign(l, r) => format!("({} *= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::DivAssign(l, r) => format!("({} /= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::ModAssign(l, r) => format!("({} %= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::ShlAssign(l, r) => format!("({} <<= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::ShrAssign(l, r) => format!("({} >>= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::BitAndAssign(l, r) => format!("({} &= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::BitXorAssign(l, r) => format!("({} ^= {})", l.to_cpp(st)?, r.to_cpp(st)?),
            Expr::BitOrAssign(l, r) => format!("({} |= {})", l.to_cpp(st)?, r.to_cpp(st)?),
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
            Expr::Ident(_, _) => todo!(),
            Expr::Add(_, _) => todo!(),
            Expr::Sub(_, _) => todo!(),
            Expr::Mul(_, _) => todo!(),
            Expr::Div(_, _) => todo!(),
            Expr::Mod(_, _) => todo!(),
            Expr::Neg(_) => todo!(),
            Expr::Pos(_) => todo!(),
            Expr::IncR(_) => todo!(),
            Expr::IncL(_) => todo!(),
            Expr::DecR(_) => todo!(),
            Expr::DecL(_) => todo!(),
            Expr::Factorial(_) => todo!(),
            Expr::Deref(_) => todo!(),
            Expr::Address(_) => todo!(),
            Expr::Eq(_, _) => todo!(),
            Expr::Ne(_, _) => todo!(),
            Expr::Lt(_, _) => todo!(),
            Expr::Le(_, _) => todo!(),
            Expr::Gt(_, _) => todo!(),
            Expr::Ge(_, _) => todo!(),
            Expr::And(_, _) => todo!(),
            Expr::Or(_, _) => todo!(),
            Expr::Not(_) => todo!(),
            Expr::BitAnd(_, _) => todo!(),
            Expr::BitOr(_, _) => todo!(),
            Expr::BitXor(_, _) => todo!(),
            Expr::BitNot(_) => todo!(),
            Expr::Shl(_, _) => todo!(),
            Expr::Shr(_, _) => todo!(),
            Expr::Member(_, _) => todo!(),
            Expr::Cast(_, _) => todo!(),
            Expr::Size(_) => todo!(),
            Expr::Align(_) => todo!(),
            Expr::Assign(_, _) => todo!(),
            Expr::AddAssign(_, _) => todo!(),
            Expr::SubAssign(_, _) => todo!(),
            Expr::MulAssign(_, _) => todo!(),
            Expr::DivAssign(_, _) => todo!(),
            Expr::ModAssign(_, _) => todo!(),
            Expr::ShlAssign(_, _) => todo!(),
            Expr::ShrAssign(_, _) => todo!(),
            Expr::BitAndAssign(_, _) => todo!(),
            Expr::BitXorAssign(_, _) => todo!(),
            Expr::BitOrAssign(_, _) => todo!(),
            Expr::Call(_, _, _) => todo!(),
            Expr::ArrMember(_, _) => todo!(),
            Expr::Tuple(_, _) => todo!(),
            Expr::Array(v, s) => {
                let size = v.len();
                let typ = v.first().unwrap().resolve_type(st)?;

                let mut errored = false;
                for val in v.iter().skip(1) {
                    let t = val.resolve_type(st)?;

                    if t != typ {
                        st.errors.push(Error::Compile(
                            "Array types do not match".to_owned(),
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

impl ComfyType<String> for Vec<Expr> {
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
