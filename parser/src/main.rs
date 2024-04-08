use clap::Parser;

use std::fs::OpenOptions;

mod parser;

use parser::literals;

use nom::error::convert_error;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    input_file: String,
}

fn main() {
    // let args = Args::parse();

    // let file = OpenOptions::new()
    //     .read(true)
    //     .open(args.input_file)
    //     .expect("Could not open file");

    let data = r#"."#;
    let val = literals::literals(&data);

    match val {
        Ok(val) => {
            println!("{:?}{:?}", val.1, val.0);
        }
        Err(nom::Err::Error(err) | nom::Err::Failure(err)) => {
            println!("{}", convert_error(data, err));
        }

        _ => unreachable!(),
    }
}
