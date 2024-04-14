use clap::Parser;
use comfy_parser::parse;
use std::fs;

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

    let _ = parse(src_file, src);
}
