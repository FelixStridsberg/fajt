use fajt_ast::LitString;

pub(crate) trait DirectivePrologueSemantics {
    fn contains_strict(&self) -> bool;
}

impl DirectivePrologueSemantics for Vec<LitString> {
    fn contains_strict(&self) -> bool {
        self.iter().any(|s| s.value == "use strict")
    }
}
