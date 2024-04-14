pub mod lexer;
mod parser;

use comfy_types::Ast;
use lexer::tokens;
pub use parser::*;

use ariadne::{Color, Label, Report, ReportKind, Source};

use chumsky::input::Input;
use chumsky::Parser as ChumskyParser;

pub fn parse(src_file: &str, src: String) -> Result<Ast, ()> {
    match tokens().parse(&src).into_result() {
        Ok(tokens) => {
            let tokens = tokens.as_slice().spanned((src.len()..src.len()).into());

            match statements().parse(tokens).into_result() {
                Ok(ast) => Ok(ast),
                Err(parse_errs) => Err(print_err!(parse_errs, src_file, &src)),
            }
        }
        Err(parse_errs) => Err(print_err!(parse_errs, src_file, &src)),
    }
}

#[macro_export]
macro_rules! print_err {
    ($errs:expr, $src_file:expr, $src:expr) => {
        $errs.into_iter().for_each(|e| {
            Report::build(ReportKind::Error, $src_file, e.span().start)
                .with_message(e.to_string())
                .with_label(
                    Label::new(($src_file, e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .print(($src_file, Source::from($src)))
                .unwrap()
        })
    };
}
