use std::fs;

use clap::Parser;

use ariadne::{Color, Label, Report, ReportKind, Source};

use chumsky::Parser as ChumskyParser;
use colored::*;
use comfy_parser::statements;
use compiler::{Compiler, Error};

mod compiler;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    input_file: String,

    #[clap(short, long)]
    output_file: String,
}

fn main() {
    let args = Args::parse();

    let src_file = &args.input_file;
    let src = fs::read_to_string(src_file).expect("Could not read file");

    let output_file = &args.output_file;

    match statements().parse(&src).into_result() {
        Ok(ast) => {
            let mut compiler = Compiler::new(ast);

            let compiled = compiler.compile(output_file);

            match compiled {
                Ok(compiled) => {
                    println!("{}", compiled);
                }
                Err(e) => e.into_iter().for_each(|e| match e {
                    Error::Compile(msg, s) => Report::build(ReportKind::Error, src_file, s.start)
                        .with_message(msg.clone())
                        .with_label(
                            Label::new((src_file, s.into_range()))
                                .with_message(msg.to_string())
                                .with_color(Color::Red),
                        )
                        .finish()
                        .print((src_file, Source::from(&src)))
                        .unwrap(),
                    Error::Clang(msg) => eprintln!("{}", msg.red().bold()),
                }),
            }
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
