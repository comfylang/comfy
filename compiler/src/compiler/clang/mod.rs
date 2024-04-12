use std::{
    fs,
    process::{self, Command, Stdio},
};

use super::{CompileError, CompileResult};

struct ClangArgs {
    input_file: String,
    output_file: String,
}

pub fn compile(src: &str, output_file: String) -> CompileResult<()> {
    let input_file = format!("{}.temp.cc", output_file);

    fs::write(&input_file, src).unwrap();

    let res = run_clang(ClangArgs {
        input_file: input_file.clone(),
        output_file,
    });

    fs::remove_file(&input_file).unwrap();

    res
}

fn run_clang(clang_args: ClangArgs) -> Result<(), CompileError> {
    let mut command = Command::new(clang_resolve());

    let mut args: Vec<&str> = vec![];

    args.push(&clang_args.input_file);
    args.push("-o");
    args.push(&clang_args.output_file);

    command
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let output = command.output().expect(&format!(
        "Failed to execute clang++ command, are you sure it's installed?"
    ));

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(CompileError(format!("Compilation failed: {error_message}")))
    } else {
        Ok(())
    }
}

fn clang_resolve() -> String {
    "clang++".to_owned()
}
