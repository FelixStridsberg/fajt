use crate::error::Result;
use crate::Error;
use fajt_ast::{FormalParameters, LitString};

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
    fn early_errors_setter(&self) -> Result<()>;
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

    fn early_errors_setter(&self) -> Result<()> {
        if self.rest.is_some() {
            return Err(Error::syntax_error(
                "Setter function parameter must not be a rest parameter".to_owned(),
                self.span.clone(),
            ));
        }

        if self.bindings.len() != 1 {
            return Err(Error::syntax_error(
                "Setter must have exactly one parameter".to_owned(),
                self.span.clone(),
            ));
        }

        Ok(())
    }
}
