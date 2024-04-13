use chumsky::span::SimpleSpan;
use comfy_types::AccessModifier;

use super::{ComfyType, CompileResult, State};

impl ComfyType<String> for AccessModifier {
    fn to_c(&self, _: &mut State) -> CompileResult<String> {
        Ok(match self {
            AccessModifier::Public(_) => "public".to_owned(),
            AccessModifier::Private(_) => "private".to_owned(),
            AccessModifier::Protected(_) => "protected".to_owned(),
        })
    }

    fn span(&self) -> SimpleSpan {
        match self {
            AccessModifier::Public(s) => *s,
            AccessModifier::Private(s) => *s,
            AccessModifier::Protected(s) => *s,
        }
    }
}
