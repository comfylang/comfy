mod clang;
use comfy_types::Ast;

mod translation;

pub use translation::*;

use self::clang::compile;

impl Compiler {
    pub fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub fn compile(&mut self, output_file: &str) -> CompileResult<String> {
        let mut state = State::new();
        let result = self.ast.to_c(&mut state)?;

        let result = format!("#include <iostream>\n#include <stdint.h>\n\n{}", result);

        compile(&result, output_file.to_owned())?;

        Ok(result)
    }
}
