pub mod literals;

use comfy_types::{Statements, Types};
use nom::{multi::many1, IResult};

fn statements(input: &str) -> IResult<&str, Statements> {
    todo!()
}

pub fn parse(input: &str) -> IResult<&str, Vec<Statements>> {
    many1(statements)(input)
}
