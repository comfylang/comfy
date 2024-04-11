use chumsky::pratt::*;
use chumsky::prelude::*;
use comfy_types::Expr;

use super::types::types;
use super::{common::ident, literals::literals, ParseError};

pub fn expressions<'a>() -> impl Parser<'a, &'a str, Expr, ParseError<'a>> {
    let id = ident().map(|s| Expr::Ident(s)).padded().boxed();
    let lit = literals().map(|l| Expr::Literal(l)).padded().boxed();
    let ty = types().map(|t| Expr::Type(t)).padded().boxed();

    let op = |c: &'a str| just(c).padded();

    let complex_expr = recursive(|expr| {
        let call = id
            .clone()
            .then(
                expr.clone()
                    .separated_by(just(','))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just('(').padded(), just(')').padded()),
            )
            .map(|(f, args)| Expr::Call(b(f), args));

        let arr_member = just("[")
            .padded()
            .then(expr.clone())
            .then(just("]").padded())
            .map(|r| Expr::ArrMember(b(r.0 .1)));

        let arr_expr = expr
            .clone()
            .separated_by(just(',').padded())
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(just('['), just(']'))
            .map(|s| Expr::Array(s));

        let tuple_expr = expr
            .clone()
            .separated_by(just(',').padded())
            .allow_trailing()
            .collect()
            .padded()
            .delimited_by(just('('), just(')'))
            .map(|s| Expr::Tuple(s));

        let atom = lit
            .or(expr.delimited_by(just("("), just(")")))
            .or(ty)
            .or(call)
            .or(arr_member)
            .or(arr_expr)
            .or(tuple_expr)
            .or(id)
            .padded();

        let pr1 = atom.pratt((
            //
            //
            infix(left(15), op("."), |l, r| Expr::Member(b(l), b(r))),
            postfix(15, op("++"), |lhs| Expr::IncR(b(lhs))),
            postfix(15, op("--"), |lhs| Expr::DecR(b(lhs))),
            postfix(15, op("!"), |lhs| Expr::Factorial(b(lhs))),
            //
            //
            infix(right(14), op("as"), |l, r| Expr::Cast(b(l), b(r))),
            prefix(14, op("++"), |rhs| Expr::IncL(b(rhs))),
            prefix(14, op("--"), |rhs| Expr::DecL(b(rhs))),
            prefix(14, op("-"), |rhs| Expr::Neg(b(rhs))),
            prefix(14, op("+"), |rhs| Expr::Pos(b(rhs))),
            prefix(14, op("!"), |rhs| Expr::Not(b(rhs))),
            prefix(14, op("~"), |rhs| Expr::BitNot(b(rhs))),
            prefix(14, op("*"), |rhs| Expr::Deref(b(rhs))),
            prefix(14, op("&"), |rhs| Expr::Address(b(rhs))),
            prefix(14, op("sizeof"), |rhs| Expr::Size(b(rhs))),
            prefix(14, op("alignof"), |rhs| Expr::Align(b(rhs))),
            //
            //
            infix(left(13), op("*"), |l, r| Expr::Mul(b(l), b(r))),
            infix(left(13), op("/"), |l, r| Expr::Div(b(l), b(r))),
            infix(left(13), op("%"), |l, r| Expr::Mod(b(l), b(r))),
            //
            //
            infix(left(12), op("+"), |l, r| Expr::Add(b(l), b(r))),
            infix(left(12), op("-"), |l, r| Expr::Sub(b(l), b(r))),
            //
            //
            infix(left(11), op("<<"), |l, r| Expr::Shl(b(l), b(r))),
            infix(left(11), op(">>"), |l, r| Expr::Shr(b(l), b(r))),
            //
            //
            infix(left(10), op("<"), |l, r| Expr::Lt(b(l), b(r))),
            infix(left(10), op("<="), |l, r| Expr::Le(b(l), b(r))),
            infix(left(10), op(">"), |l, r| Expr::Gt(b(l), b(r))),
            infix(left(10), op(">="), |l, r| Expr::Ge(b(l), b(r))),
        ));

        let pr2 = pr1.pratt((
            infix(left(9), op("=="), |l, r| Expr::Eq(b(l), b(r))),
            infix(left(9), op("!="), |l, r| Expr::Ne(b(l), b(r))),
            //
            infix(left(8), op("&"), |l, r| Expr::BitAnd(b(l), b(r))),
            //
            infix(left(7), op("^"), |l, r| Expr::BitXor(b(l), b(r))),
            //
            infix(left(6), op("|"), |l, r| Expr::BitOr(b(l), b(r))),
            //
            infix(left(5), op("&&"), |l, r| Expr::And(b(l), b(r))),
            //
            infix(left(4), op("||"), |l, r| Expr::Or(b(l), b(r))),
            //
            infix(right(3), op("="), |l, r| Expr::Assign(b(l), b(r))),
            infix(right(3), op("+="), |l, r| Expr::AddAssign(b(l), b(r))),
            infix(right(3), op("-="), |l, r| Expr::SubAssign(b(l), b(r))),
            infix(right(3), op("*="), |l, r| Expr::MulAssign(b(l), b(r))),
            infix(right(3), op("/="), |l, r| Expr::DivAssign(b(l), b(r))),
            infix(right(3), op("%="), |l, r| Expr::ModAssign(b(l), b(r))),
            infix(right(3), op("<<="), |l, r| Expr::ShlAssign(b(l), b(r))),
            infix(right(3), op(">>="), |l, r| Expr::ShrAssign(b(l), b(r))),
            infix(right(3), op("&="), |l, r| Expr::BitAndAssign(b(l), b(r))),
            infix(right(3), op("^="), |l, r| Expr::BitXorAssign(b(l), b(r))),
            infix(right(3), op("|="), |l, r| Expr::BitOrAssign(b(l), b(r))),
        ));

        pr2
    });

    complex_expr.padded()
}

fn b<T>(a: T) -> Box<T> {
    Box::new(a)
}
