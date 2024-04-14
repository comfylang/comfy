mod common;
mod expressions;
mod literals;
mod statements;
mod types;

use comfy_types::tokens::Kind;
pub use common::ident;
pub use expressions::expressions;

pub use statements::statements;
pub use types::types;

use chumsky::prelude::*;

pub type ParseError<'a> = extra::Err<Rich<'a, char>>;
pub type TokenParseError<'a> = extra::Err<Rich<'a, Kind, SimpleSpan>>;
