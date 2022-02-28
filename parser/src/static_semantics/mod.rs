use crate::error::Result;
use fajt_ast::{FormalParameters, LitString};
use crate::Error;

pub(crate) trait DirectivePrologueSemantics {
    fn contains_strict(&self) -> bool;
}

impl DirectivePrologueSemantics for Vec<LitString> {
    fn contains_strict(&self) -> bool {
        self.iter().any(|s| s.value == "use strict")
    }
}

pub(crate) trait FormalParametersSemantics {
    fn early_errors_getter(&self) -> Result<()>;
}

impl FormalParametersSemantics for FormalParameters {
    fn early_errors_getter(&self) -> Result<()> {
        if !self.bindings.is_empty() || self.rest.is_some() {
            return Err(Error::syntax_error(
                "Getter must not have any formal parameters".to_owned(),
                self.span.clone(),
            ));
        }

        Ok(())
    }
}
