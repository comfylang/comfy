pub mod common;
pub mod literals;
pub mod types;

use chumsky::prelude::*;

pub type ParseError<'a> = extra::Err<Rich<'a, char>>;
