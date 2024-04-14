use comfy_types::Ast;
use std::time::{Duration, Instant};

mod clang;
mod translation;
use clang::compile;

pub use translation::*;

use crate::cli::Args;

pub struct CompilerOut {
    pub code: String,
    pub compile_time: Duration,
    pub translation_time: Duration,
}

impl Compiler {
    pub fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub fn compile(&mut self, args: Args) -> Result<CompilerOut, Vec<Error>> {
        let output_file = args.output_file.unwrap_or("a.out".to_owned());

        let mut state = State::new();

        let translation_time = Instant::now();
        let result = self.ast.to_cpp(&mut state);
        let translation_time = translation_time.elapsed();

        if state.errors.len() > 0 || result.is_err() {
            if let Err(e) = result {
                state.errors.push(e);
            }

            return Err(state.errors);
        }

        let code = result.unwrap();
        let code = format!("#include <iostream>\n#include <stdint.h>\n\n{}", code);

        let compile_time = Instant::now();

        let compiled = if args.dry_run {
            Ok(())
        } else {
            compile(&code, output_file.to_owned())
        };

        let compile_time = compile_time.elapsed();

        if let Err(e) = compiled {
            state.errors.push(e);
            return Err(state.errors);
        } else {
            return Ok(CompilerOut {
                code,
                compile_time,
                translation_time,
            });
        }
    }
}
