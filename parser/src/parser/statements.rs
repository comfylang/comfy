use chumsky::prelude::*;
use comfy_types::AccessModifier;
use comfy_types::Statements;

use super::common::access_modifier;
use super::common::assignment;
use super::common::decl_args;
use super::common::fn_type_descriptor;
use super::common::justp;
use super::common::type_descriptor;
use super::ParseError;
use super::{expressions, ident};

pub fn statements<'a>() -> impl Parser<'a, &'a str, Vec<Statements>, ParseError<'a>> {
    recursive(|stmts| {
        let expr_statement = expressions()
            .then_ignore(justp(";"))
            .map_with(|expr, e| Statements::ExpressionStatement(expr, e.span()));

        let let_statement = justp("let")
            .ignore_then(ident().padded())
            .then(type_descriptor())
            .then(assignment())
            .then_ignore(justp(";"))
            .map_with(|((name, ty), expr), e| Statements::LetStatement(name, ty, expr, e.span()));

        let function_declaration = access_modifier()
            .or_not()
            .then_ignore(justp("fn"))
            .then(ident())
            .then(decl_args().delimited_by(justp("("), justp(")")))
            .then(fn_type_descriptor())
            .then(stmts.clone().delimited_by(justp("{"), justp("}")))
            .map_with(|((((access_modifier, name), args), ty), body), e| {
                Statements::FunctionDeclaration(
                    access_modifier.unwrap_or(AccessModifier::Private(e.span())),
                    name,
                    args,
                    ty,
                    body,
                    e.span(),
                )
            });

        let return_statement = choice((
            justp("return")
                .ignore_then(expressions())
                .then_ignore(justp(";")),
            expressions(),
        ))
        .map_with(|expr, e| Statements::ReturnStatement(expr, e.span()));

        choice((
            function_declaration,
            expr_statement,
            let_statement,
            return_statement,
        ))
        .repeated()
        .collect::<Vec<_>>()
        .padded()
        .boxed()
    })
}
