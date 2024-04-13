use std::fs::{self};

use clap::Parser;

mod parser;
use ariadne::{Color, Label, Report, ReportKind, Source};

use parser::statements;

use chumsky::Parser as ChumskyParser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();

    let src_file = &args.input_file;
    let src = fs::read_to_string(src_file).expect("Could not read file");

    match statements().parse(&src).into_result() {
        Ok(ast) => {
            println!("AST: {:#?}", ast);
        }
        Err(parse_errs) => parse_errs.into_iter().for_each(|e| {
            Report::build(ReportKind::Error, src_file, e.span().start)
                .with_message(e.to_string())
                .with_label(
                    Label::new((src_file, e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .print((src_file, Source::from(&src)))
                .unwrap()
        }),
    };
}
