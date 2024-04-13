use comfy_types::Expr;

use super::{CompileError, CompileResult, Compiler, State, ToC};

impl ToC<String> for Expr {
    fn to_c(&self, st: &mut State) -> CompileResult<String> {
        Ok(match self {
            Expr::Literal(l) => l.to_c(st)?,
            Expr::Type(t) => {
                let ct = t.to_c(st)?;

                if ct.1 .0 {
                    return Err(CompileError("Cannot cast to array-like type".to_owned()));
                }

                ct.0
            }
            Expr::Ident(i) => i.into(),
            Expr::Add(r, l) => format!("({} + {})", l.to_c(st)?, r.to_c(st)?),
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
            Expr::Call(l, r, s) => format!("{}({})", l.to_c(st)?, r.to_owned().to_c(st)?),
            Expr::ArrMember(_, _) => todo!(),
            Expr::Tuple(_, _) => todo!(),
            Expr::Array(_, _) => todo!(),
            Expr::Unknown => todo!(),
        })
    }
}

impl ToC<String> for Vec<Expr> {
    fn to_c(&self, st: &mut State) -> CompileResult<String> {
        Ok(self
            .iter()
            .map(|s| s.to_c(st))
            .collect::<Result<Vec<_>, _>>()?
            .join(","))
    }
}