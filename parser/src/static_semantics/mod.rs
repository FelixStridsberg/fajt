#[macro_use]
mod macros;
mod literal;

pub(crate) use literal::*;

use crate::error::Result;
use crate::{Context, Error};
use fajt_ast::{
    AssignmentOperator, BindingPattern, Expr, ExprLiteral, FormalParameters, LitString, Literal,
    Spanned,
};

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

        fn early_errors_left_hand_side_expr(
            &self,
            context: &Context,
            operator: &AssignmentOperator,
        ) -> Result<()> {
            if operator == &AssignmentOperator::Assign {
                match self {
                    Expr::Literal(ExprLiteral {
                        literal: Literal::Array(array),
                        ..
                    }) => {
                        return array.assert_covers_assignment_pattern();
                    }
                    Expr::Literal(ExprLiteral {
                        literal: Literal::Object(object),
                        ..
                    }) => {
                        return object.assert_covers_assignment_pattern();
                    }
                    _ => {}
                }
            }

            if !self.is_assignment_target_type_simple(context)? {
                return Err(Error::syntax_error(
                    "Invalid left-hand side assignment".to_owned(),
                    self.span().clone(),
                ));
            }

            Ok(())
        }

        fn early_errors_update_expr_argument(&self, context: &Context) -> Result<()> {
            if !self.is_assignment_target_type_simple(context)? {
                return Err(Error::syntax_error(
                    "Invalid update expression argument".to_owned(),
                    self.span().clone(),
                ));
            }

            Ok(())
        }

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
    impl trait DirectivePrologueSemantics for &[LitString] {
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
        fn is_simple(&self) -> bool {
            if self.rest.is_some() {
                return false;
            }

            for binding in &self.bindings {
                if binding.initializer.is_some() {
                    return false;
                }

                if matches!(binding.pattern, BindingPattern::Object(_) | BindingPattern::Array(_)) {
                    return false;
                }
            }

            true
        }

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

        fn early_errors_method(&self, body_directives: &[LitString]) -> Result<()> {
            self.early_errors_forbidden_use_strict(body_directives)?;

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

        fn early_errors_setter(&self, body_directives: &[LitString]) -> Result<()> {
            self.early_errors_forbidden_use_strict(body_directives)?;

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

        fn early_errors_forbidden_use_strict(&self, body_directives: &[LitString]) -> Result<()> {
            if !self.is_simple() && body_directives.contains_strict() {
                return Err(Error::syntax_error(
                    "Only name parameters allowed in method with \"use strict\"".to_owned(),
                    self.span.clone()
                ))
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
