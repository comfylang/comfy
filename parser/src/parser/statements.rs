use chumsky::prelude::*;
use comfy_types::tokens::Kind;
use comfy_types::tokens::TokenInput;
use comfy_types::AccessModifier;
use comfy_types::Statements;

use super::ParseError;

use super::common::access_modifier;
use super::common::assignment;
use super::common::decl_args;
use super::common::fn_type_descriptor;

use super::common::type_descriptor;
use super::{expressions, ident};

pub fn statements<'a>() -> impl Parser<'a, TokenInput<'a>, Vec<Statements>, ParseError<'a>> {
    recursive(|stmts| {
        let expr_statement = expressions()
            .then_ignore(just(Kind::Semicolon))
            .map_with(|expr, e| Statements::ExpressionStatement(expr, e.span()))
            .labelled("expression statement");

        let let_statement = just(Kind::Let)
            .ignore_then(ident())
            .then(type_descriptor())
            .then(assignment())
            .then_ignore(just(Kind::Semicolon))
            .map_with(|((name, ty), expr), e| Statements::LetStatement(name, ty, expr, e.span()))
            .labelled("let statement");

        let function_declaration = access_modifier()
            .or_not()
            .then_ignore(just(Kind::Fn))
            .then(ident())
            .then(decl_args().delimited_by(just(Kind::LParen), just(Kind::RParen)))
            .then(fn_type_descriptor())
            .then(
                stmts
                    .clone()
                    .delimited_by(just(Kind::LAngle), just(Kind::RAngle)),
            )
            .map_with(|((((access_modifier, name), args), ty), body), e| {
                Statements::FunctionDeclaration(
                    access_modifier.unwrap_or(AccessModifier::Private(e.span())),
                    name,
                    args,
                    ty,
                    body,
                    e.span(),
                )
            })
            .labelled("function declaration");

        let return_statement = choice((
            just(Kind::Return)
                .ignore_then(expressions())
                .then_ignore(just(Kind::Semicolon)),
            expressions(),
        ))
        .map_with(|expr, e| Statements::ReturnStatement(expr, e.span()))
        .labelled("return statement");

        choice((
            function_declaration,
            expr_statement,
            let_statement,
            return_statement,
        ))
        .repeated()
        .collect::<Vec<Statements>>()
        .labelled("statements")
        .boxed()
    })
}
