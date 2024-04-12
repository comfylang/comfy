use comfy_types::AccessModifier;

use super::{CompileResult, State, ToC};

impl ToC<String> for AccessModifier {
    fn to_c(&self, _: &mut State) -> CompileResult<String> {
        Ok(match self {
            AccessModifier::Public => "public".to_owned(),
            AccessModifier::Private => "private".to_owned(),
            AccessModifier::Protected => "protected".to_owned(),
        })
    }
}
