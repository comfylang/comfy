mod common;
mod expressions;
mod literals;
mod statements;
mod types;

use comfy_types::tokens::Kind;
pub use common::ident;
pub use expressions::expression;

pub use statements::statements;
pub use types::types;

use chumsky::prelude::*;

pub type LexError<'a> = extra::Err<Rich<'a, char>>;
pub type ParseError<'a> = extra::Err<Rich<'a, Kind, SimpleSpan>>;
