mod clang;
use comfy_types::Ast;

mod translation;

pub use translation::*;

use self::clang::compile;

impl Compiler {
    pub fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub fn compile(&mut self, output_file: &str) -> Result<String, Vec<Error>> {
        let mut state = State::new();

        let result = self.ast.to_cpp(&mut state);

        if state.errors.len() > 0 || result.is_err() {
            if let Err(e) = result {
                state.errors.push(e);
            }

            return Err(state.errors);
        }

        let result = result.unwrap();

        let result = format!("#include <iostream>\n#include <stdint.h>\n\n{}", result);

        let compiled = compile(&result, output_file.to_owned());

        if let Err(e) = compiled {
            state.errors.push(e);
            return Err(state.errors);
        } else {
            return Ok(result);
        }
    }
}
