use chumsky::span::SimpleSpan;
use comfy_types::Expr;

use super::{ComfyType, CompileResult, Error, State};

impl ComfyType<String> for Expr {
    fn to_c(&self, st: &mut State) -> CompileResult<String> {
        Ok(match self {
            Expr::Literal(l) => l.to_c(st)?,
            Expr::Type(t) => {
                let ct = t.to_c(st)?;
                let span = self.span();

                if ct.1 .0 {
                    return Err(Error::Compile(
                        "Cannot cast to array-like type".to_owned(),
                        span,
                    ));
                }

                ct.0
            }
            Expr::Ident(i, _) => i.into(),
            Expr::Add(l, r) => format!("({} + {})", l.to_c(st)?, r.to_c(st)?),
            Expr::Sub(l, r) => format!("({} - {})", l.to_c(st)?, r.to_c(st)?),
            Expr::Mul(l, r) => format!("({} * {})", l.to_c(st)?, r.to_c(st)?),
            Expr::Div(l, r) => format!("({} / {})", l.to_c(st)?, r.to_c(st)?),
            Expr::Mod(l, r) => format!("({} % {})", l.to_c(st)?, r.to_c(st)?),
            Expr::Neg(_) => todo!(),
            Expr::Pos(_) => todo!(),
            Expr::IncR(_) => todo!(),
            Expr::IncL(_) => todo!(),
            Expr::DecR(_) => todo!(),
            Expr::DecL(_) => todo!(),
            Expr::Factorial(_) => todo!(),
            Expr::Deref(_) => todo!(),
            Expr::Address(_) => todo!(),
            Expr::Eq(l, r) => todo!(),
            Expr::Ne(l, r) => todo!(),
            Expr::Lt(l, r) => todo!(),
            Expr::Le(l, r) => todo!(),
            Expr::Gt(l, r) => todo!(),
            Expr::Ge(l, r) => todo!(),
            Expr::And(l, r) => todo!(),
            Expr::Or(l, r) => todo!(),
            Expr::Not(_) => todo!(),
            Expr::BitAnd(l, r) => todo!(),
            Expr::BitOr(l, r) => todo!(),
            Expr::BitXor(l, r) => todo!(),
            Expr::BitNot(_) => todo!(),
            Expr::Shl(l, r) => todo!(),
            Expr::Shr(l, r) => todo!(),
            Expr::Member(l, r) => todo!(),
            Expr::Cast(l, r) => format!("(({}) {} )", r.to_c(st)?, l.to_c(st)?),
            Expr::Size(_) => todo!(),
            Expr::Align(_) => todo!(),
            Expr::Assign(l, r) => todo!(),
            Expr::AddAssign(l, r) => todo!(),
            Expr::SubAssign(l, r) => todo!(),
            Expr::MulAssign(l, r) => todo!(),
            Expr::DivAssign(l, r) => todo!(),
            Expr::ModAssign(l, r) => todo!(),
            Expr::ShlAssign(l, r) => todo!(),
            Expr::ShrAssign(l, r) => todo!(),
            Expr::BitAndAssign(l, r) => todo!(),
            Expr::BitXorAssign(l, r) => todo!(),
            Expr::BitOrAssign(l, r) => todo!(),
            Expr::Call(l, r, _) => format!("{}({})", l.to_c(st)?, r.to_owned().to_c(st)?),
            Expr::ArrMember(_, _) => todo!(),
            Expr::Tuple(_, _) => todo!(),
            Expr::Array(_, _) => todo!(),
            Expr::Unknown => todo!(),
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
            Expr::ArrMember(_, s) => *s,
            Expr::Tuple(_, s) => *s,
            Expr::Array(_, s) => *s,
            Expr::Unknown => SimpleSpan::new(0, 0),
        }
    }
}

impl ComfyType<String> for Vec<Expr> {
    fn to_c(&self, st: &mut State) -> CompileResult<String> {
        Ok(self
            .iter()
            .map(|s| s.to_c(st))
            .collect::<Result<Vec<_>, _>>()?
            .join(","))
    }

    fn span(&self) -> SimpleSpan {
        let start = self.first().unwrap().span().start;
        let end = self.last().unwrap().span().end;

        SimpleSpan::new(start, end)
    }
}
