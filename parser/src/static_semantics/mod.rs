#[macro_use]
mod macros;

use crate::error::Result;
use crate::Error;
use fajt_ast::{BindingPattern, FormalParameters, LitString};

impl_trait!(
    impl trait DirectivePrologueSemantics for Vec<LitString> {
        fn contains_strict(&self) -> bool {
            self.iter().any(|s| s.value == "use strict")
        }
    }
);

impl_trait!(
    impl trait BindingPatternSemantics for BindingPattern {
        fn get_bound_names(&self) -> Vec<&str> {
            match &self {
                BindingPattern::Ident(ident) => {
                    vec![ident.name.as_ref()]
                }
                BindingPattern::Array(_arr) => {
                    todo!()
                }
                BindingPattern::Object(_obj) => {
                    todo!()
                }
            }
        }
    }
);

impl_trait!(
    impl trait FormalParametersSemantics for FormalParameters {
        fn bound_names(&self) -> Vec<&str> {
            let mut names: Vec<&str> = self
                .bindings
                .iter()
                .flat_map(|binding| binding.pattern.get_bound_names())
                .collect();

            if let Some(rest) = &self.rest {
                names.append(&mut rest.as_ref().get_bound_names());
            }

            names
        }

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

        fn early_errors_method(&self) -> Result<()> {
            let mut bound_names = self.bound_names();
            bound_names.sort_unstable();

            let first_duplicate = get_first_duplicate(&bound_names);

            if let Some(duplicate) = first_duplicate {
                return Err(Error::syntax_error(
                    format!(
                        "Found duplicate parameter '{}', duplicates not allowed here",
                        duplicate
                    ),
                    self.span.clone(),
                ));
            }

            Ok(())
        }
    }
);

/// Assumes the `list` is sorted.
fn get_first_duplicate<'a>(list: &[&'a str]) -> Option<&'a str> {
    let mut iter = list.iter().peekable();
    while let Some(item) = iter.next() {
        if let Some(peek) = iter.peek() {
            if item == *peek {
                return Some(item);
            }
        }
    }

    None
}
