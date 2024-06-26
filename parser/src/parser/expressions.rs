use chumsky::pratt::*;
use chumsky::prelude::*;
use comfy_types::tokens::Kind;
use comfy_types::tokens::TokenInput;
use comfy_types::Expr;
use comfy_utils::b;

use super::common::cpp_code;
use super::ParseError;

use super::types::types;
use super::{common::ident, literals::literals};

pub fn expression<'a>() -> impl Parser<'a, TokenInput<'a>, Expr, ParseError<'a>> {
    let cpp_code = cpp_code()
        .map_with(|s, e| Expr::CppCode(s, e.span()))
        .boxed();

    let id = ident().map_with(|s, e| Expr::Ident(s, e.span())).boxed();

    let lit = literals().map(|l| Expr::Literal(l)).boxed();

    let ty = types().map(|t| Expr::Type(t)).boxed();

    let op = |c: Kind| just(c).boxed();

    let complex_expr = recursive(|expr| {
        let arr_expr = expr
            .clone()
            .separated_by(just(Kind::Comma))
            .allow_trailing()
            .collect()
            .delimited_by(just(Kind::LSquare), just(Kind::RSquare))
            .map_with(|s, e| Expr::Array(s, e.span()))
            .labelled("array expression");

        let tuple_expr = expr
            .clone()
            .separated_by(just(Kind::Comma))
            .allow_trailing()
            .collect()
            .delimited_by(just(Kind::LParen), just(Kind::RParen))
            .map_with(|s, e| Expr::Tuple(s, e.span()))
            .labelled("tuple expression");

        let atom = lit
            .or(expr
                .clone()
                .delimited_by(just(Kind::LParen), just(Kind::RParen)))
            .or(ty)
            .or(arr_expr)
            .or(tuple_expr)
            .or(cpp_code)
            .or(id)
            .boxed();

        let call = atom
            .foldl_with(
                expr.clone()
                    .separated_by(just(Kind::Comma))
                    .allow_trailing()
                    .collect()
                    .delimited_by(just(Kind::LParen), just(Kind::RParen))
                    .repeated(),
                |f, args, e| Expr::Call(b(f), args, e.span()),
            )
            .labelled("call expression");

        let arr_member = call
            .foldl(
                expr.clone()
                    .delimited_by(just(Kind::LSquare), just(Kind::RSquare))
                    .repeated(),
                |l, r| Expr::ArrMember(b(l), b(r)),
            )
            .labelled("call expression");

        let prat = arr_member
            .pratt((
                //
                //
                infix(left(15), op(Kind::Dot), |l, r| Expr::Member(b(l), b(r))),
                postfix(15, op(Kind::DoublePlus), |lhs| Expr::IncR(b(lhs))),
                postfix(15, op(Kind::DoubleMinus), |lhs| Expr::DecR(b(lhs))),
                postfix(15, op(Kind::ExclamationMark), |lhs| Expr::Factorial(b(lhs))),
                //
                //
                infix(right(14), op(Kind::As), |l, r| Expr::Cast(b(l), b(r))),
                prefix(14, op(Kind::DoublePlus), |rhs| Expr::IncL(b(rhs))),
                prefix(14, op(Kind::DoubleMinus), |rhs| Expr::DecL(b(rhs))),
                prefix(14, op(Kind::Minus), |rhs| Expr::Neg(b(rhs))),
                prefix(14, op(Kind::Plus), |rhs| Expr::Pos(b(rhs))),
                prefix(14, op(Kind::ExclamationMark), |rhs| Expr::Not(b(rhs))),
                prefix(14, op(Kind::Tilde), |rhs| Expr::BitNot(b(rhs))),
                prefix(14, op(Kind::Star), |rhs| Expr::Deref(b(rhs))),
                prefix(14, op(Kind::Ampersand), |rhs| Expr::Address(b(rhs))),
                prefix(14, op(Kind::Sizeof), |rhs| Expr::Size(b(rhs))),
                prefix(14, op(Kind::Alignof), |rhs| Expr::Align(b(rhs))),
                //
                //
                infix(left(13), op(Kind::Star), |l, r| Expr::Mul(b(l), b(r))),
                infix(left(13), op(Kind::Slash), |l, r| Expr::Div(b(l), b(r))),
                infix(left(13), op(Kind::Percent), |l, r| Expr::Mod(b(l), b(r))),
                //
                //
                infix(left(12), op(Kind::Plus), |l, r| Expr::Add(b(l), b(r))),
                infix(left(12), op(Kind::Minus), |l, r| Expr::Sub(b(l), b(r))),
                //
                //
                infix(left(11), op(Kind::LeftShift), |l, r| Expr::Shl(b(l), b(r))),
                infix(left(11), op(Kind::RightShift), |l, r| Expr::Shr(b(l), b(r))),
                //
                //
                infix(left(10), op(Kind::Less), |l, r| Expr::Lt(b(l), b(r))),
                infix(left(10), op(Kind::LessEqual), |l, r| Expr::Le(b(l), b(r))),
                infix(left(10), op(Kind::Greater), |l, r| Expr::Gt(b(l), b(r))),
                infix(left(10), op(Kind::GreaterEqual), |l, r| {
                    Expr::Ge(b(l), b(r))
                }),
            ))
            .pratt((
                infix(left(9), op(Kind::DoubleEqual), |l, r| Expr::Eq(b(l), b(r))),
                infix(left(9), op(Kind::NotEqual), |l, r| Expr::Ne(b(l), b(r))),
                //
                infix(left(8), op(Kind::Ampersand), |l, r| {
                    Expr::BitAnd(b(l), b(r))
                }),
                //
                infix(left(7), op(Kind::Caret), |l, r| Expr::BitXor(b(l), b(r))),
                //
                infix(left(6), op(Kind::Pipe), |l, r| Expr::BitOr(b(l), b(r))),
                //
                infix(left(5), op(Kind::DoubleAmp), |l, r| Expr::And(b(l), b(r))),
                //
                infix(left(4), op(Kind::DoublePipe), |l, r| Expr::Or(b(l), b(r))),
                //
                infix(right(3), op(Kind::Assign), |l, r| Expr::Assign(b(l), b(r))),
                infix(right(3), op(Kind::PlusAssign), |l, r| {
                    Expr::AddAssign(b(l), b(r))
                }),
                infix(right(3), op(Kind::MinusAssign), |l, r| {
                    Expr::SubAssign(b(l), b(r))
                }),
                infix(right(3), op(Kind::StarAssign), |l, r| {
                    Expr::MulAssign(b(l), b(r))
                }),
                infix(right(3), op(Kind::SlashAssign), |l, r| {
                    Expr::DivAssign(b(l), b(r))
                }),
                infix(right(3), op(Kind::PercentAssign), |l, r| {
                    Expr::ModAssign(b(l), b(r))
                }),
                infix(right(3), op(Kind::LeftShiftAssign), |l, r| {
                    Expr::ShlAssign(b(l), b(r))
                }),
                infix(right(3), op(Kind::RightShiftAssign), |l, r| {
                    Expr::ShrAssign(b(l), b(r))
                }),
                infix(right(3), op(Kind::AmpersandAssign), |l, r| {
                    Expr::BitAndAssign(b(l), b(r))
                }),
                infix(right(3), op(Kind::CaretAssign), |l, r| {
                    Expr::BitXorAssign(b(l), b(r))
                }),
                infix(right(3), op(Kind::PipeAssign), |l, r| {
                    Expr::BitOrAssign(b(l), b(r))
                }),
            ));

        prat
    });

    complex_expr.labelled("expression")
}
