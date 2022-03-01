#[macro_use]
mod macros;

use crate::error::Result;
use crate::{Context, Error};
use fajt_ast::{
    ArrayElement, BindingPattern, Expr, FormalParameters, LitArray, LitObject, LitString,
    PropertyDefinition, Spanned,
};

impl_trait!(
    impl trait LitArraySemantics for LitArray {
        /// Returns `Err` if not covering `ArrayAssignment`.
        fn assert_covers_assignment_pattern(&self) -> Result<()> {
            let mut elements = self.elements.iter().peekable();

            while let Some(element) = elements.next() {
                if elements.peek().is_none() {
                    break;
                }

                if let ArrayElement::Spread(spread) = element {
                    return Err(Error::syntax_error(
                        "Rest element must be last element".to_owned(),
                        spread.span().clone(),
                    ));
                }
            }

            Ok(())
        }
    }
);

impl_trait!(
    impl trait LitObjectSemantics for LitObject {

        /// Returns `Err` if not covering `ObjectAssignment`.
        fn assert_covers_assignment_pattern(&self) -> Result<()> {
            let mut props = self.props.iter().peekable();
            while let Some(prop) = props.next() {
                if let PropertyDefinition::Method(method) = prop {
                    return Err(Error::syntax_error(
                        "Invalid destructuring assignment target".to_owned(),
                        method.span.clone(),
                    ));
                }

                if props.peek().is_some() {
                    if let PropertyDefinition::Spread(spread) = prop {
                        return Err(Error::syntax_error(
                            "Rest element must be last element".to_owned(),
                            spread.span().clone(),
                        ));
                    }
                }
            }

            Ok(())
        }
    }
);

impl_trait!(
    impl trait ExprSemantics for Expr {
        /// Returns true if `AssignmentTargetType` for `expr` is simple.
        fn is_assignment_target_type_simple(&self, context: &Context) -> Result<bool> {
            Ok(match self {
                Expr::IdentRef(ident) => {
                    if context.is_strict && (ident.name == "arguments" || ident.name == "eval") {
                        return Err(Error::syntax_error(
                            "Unexpected `eval` or `arguments` in strict mode".to_owned(),
                            self.span().clone(),
                        ));
                    } else {
                        true
                    }
                }
                Expr::Member(_) => true,
                _ => false,
            })
        }

        /// Early error on invalid update expression argument.
        fn early_errors_update_expr_argument(&self, context: &Context) -> Result<()> {
            if !self.is_assignment_target_type_simple(&context)? {
                return Err(Error::syntax_error(
                    "Invalid update expression argument".to_owned(),
                    self.span().clone(),
                ));
            }

            Ok(())
        }

        /// Early error on invalid delete argument.
        fn early_errors_unary_delete(&self, context: &Context) -> Result<()> {
            if !context.is_strict {
                return Ok(());
            }

            match self {
                Expr::IdentRef(ident) => {
                    return Err(Error::syntax_error(
                        "Delete of an unqualified identifier in strict mode".to_owned(),
                        ident.span.clone(),
                    ));
                }
                Expr::Parenthesized(parenthesized) => {
                    return parenthesized.expression.early_errors_unary_delete(context);
                }
                _ => {}
            }

            Ok(())
        }
    }
);

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
