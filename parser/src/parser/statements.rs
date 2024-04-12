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
            .map(Statements::ExpressionStatement);

        let let_statement = justp("let")
            .ignore_then(ident().padded())
            .then(type_descriptor())
            .then(assignment())
            .then_ignore(justp(";"))
            .map(|((name, ty), expr)| Statements::LetStatement(name, ty, expr));

        let function_declaration = access_modifier()
            .or_not()
            .then_ignore(justp("fn"))
            .then(ident())
            .then(decl_args().delimited_by(justp("("), justp(")")))
            .then(fn_type_descriptor())
            .then(stmts.clone().delimited_by(justp("{"), justp("}")))
            .map(|((((access_modifier, name), args), ty), body)| {
                Statements::FunctionDeclaration(
                    access_modifier.unwrap_or(AccessModifier::Private),
                    name,
                    args,
                    ty,
                    body,
                )
            });

        let return_statement = choice((
            justp("return")
                .ignore_then(expressions())
                .then_ignore(justp(";")),
            expressions(),
        ))
        .map(Statements::ReturnStatement);

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
