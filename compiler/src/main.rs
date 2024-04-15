use std::fs;

use ariadne::{Color, Label, Report, ReportKind, Source};

use clap::Parser;
use colored::*;
use comfy_parser::parse;
use comfy_utils::inc_indent;
use compiler::{Compiler, Error};

mod cli;
mod compiler;

fn main() {
    let args = cli::Args::parse();

    let src_file = &args.input_file.clone();
    let src = fs::read_to_string(src_file).expect("Could not read file");

    let ast = parse(src_file, src.clone());

    if let Ok(ast) = ast {
        let mut compiler = Compiler::new(ast);

        let compiled = compiler.compile(args.clone());

        match compiled {
            Ok(compiled) => {
                if args.verbose {
                    println!(
                        "\n{}\n \n\n{:#?}\n",
                        "State:".bold().green(),
                        compiled.state
                    );
                    println!(
                        "\n{} \n\n{}\n",
                        "Translated code:".bold().green(),
                        inc_indent(compiled.code)
                    );
                    println!(
                        "{} {}s",
                        "Translation time:".bold().green(),
                        compiled.translation_time.as_secs_f64()
                    );
                    println!(
                        "{} {}s",
                        "Compilation time:".bold().green(),
                        compiled.compile_time.as_secs_f64()
                    );
                }
            }
            Err(e) => e.into_iter().for_each(|e| match e {
                Error::Compile(msg, s) => Report::build(ReportKind::Error, src_file, s.start)
                    .with_code("Compiler")
                    .with_message(msg.clone())
                    .with_label(
                        Label::new((src_file, s.into_range()))
                            .with_message(msg.to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print((src_file, Source::from(&src)))
                    .unwrap(),
                Error::Clang(msg) => eprintln!("{} {}", "[Clang]".red().bold(), msg.bold()),
            }),
        }
    }
}
