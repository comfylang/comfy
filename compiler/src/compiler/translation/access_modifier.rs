use comfy_types::AccessModifier;

use super::{CompileResult, State, ToC};

impl ToC<String> for AccessModifier {
    fn to_c(&self, _: &mut State) -> CompileResult<String> {
        Ok(match self {
            AccessModifier::Public(_) => "public".to_owned(),
            AccessModifier::Private(_) => "private".to_owned(),
            AccessModifier::Protected(_) => "protected".to_owned(),
        })
    }
}
