use chumsky::span::SimpleSpan;
use comfy_types::{AccessModifier, Type};

use super::{ComfyType, CompileResult, State};

impl ComfyType<String> for AccessModifier {
    fn to_cpp(&self, _: &mut State) -> CompileResult<String> {
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

    fn resolve_type(&self, _: &mut State) -> CompileResult<Type> {
        Ok(Type::Unknown(self.span()))
    }
}
