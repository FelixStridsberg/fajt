use crate::error::Result;
use crate::{Context, Error};
use fajt_ast::{
    ArrayElement, AssignmentOperator, Expr, ExprLiteral, LitArray, LitObject, Literal,
    PropertyDefinition, Spanned,
};

pub struct StaticSemantics {
    context: Context,
}

impl StaticSemantics {
    pub fn with_context(context: Context) -> Self {
        StaticSemantics { context }
    }

    /// Validates if expr is allowed as left-hand side for the given operator.
    pub fn validate_left_side_expr(
        &self,
        expr: &Expr,
        operator: &AssignmentOperator,
    ) -> Result<()> {
        if operator == &AssignmentOperator::Assign {
            match expr {
                Expr::Literal(ExprLiteral {
                    literal: Literal::Array(array),
                    ..
                }) => {
                    return self.validate_array_literal_cover_assignment(array);
                }
                Expr::Literal(ExprLiteral {
                    literal: Literal::Object(object),
                    ..
                }) => {
                    return self.validate_object_literal_cover_assignment(object);
                }
                _ => {}
            }
        }

        if !self.is_assignment_target_type_simple(expr)? {
            return Err(Error::syntax_error(
                "Invalid left-hand side assignment".to_owned(),
                expr.span().clone(),
            ));
        }

        Ok(())
    }

    /// Validates argument of delete unary expression.
    pub(crate) fn validate_delete_argument(&self, argument: &Expr) -> Result<()> {
        if !self.context.is_strict {
            return Ok(());
        }

        match argument {
            Expr::IdentRef(ident) => {
                return Err(Error::syntax_error(
                    "Delete of an unqualified identifier in strict mode".to_owned(),
                    ident.span.clone(),
                ));
            }
            Expr::Parenthesized(parenthesized) => {
                return self.validate_delete_argument(parenthesized.expression.as_ref());
            }
            _ => {}
        }

        Ok(())
    }

    /// Returns true if `AssignmentTargetType` for `expr` is simple.
    fn is_assignment_target_type_simple(&self, expr: &Expr) -> Result<bool> {
        Ok(match expr {
            Expr::IdentRef(ident) => {
                if self.context.is_strict && (ident.name == "arguments" || ident.name == "eval") {
                    return Err(Error::syntax_error(
                        "Unexpected `eval` or `arguments` in strict mode".to_owned(),
                        expr.span().clone(),
                    ));
                } else {
                    true
                }
            }
            Expr::Member(_) => true,
            _ => false,
        })
    }

    /// Validates that `ArrayLiteral` covers `ArrayAssignment`.
    fn validate_array_literal_cover_assignment(&self, array: &LitArray) -> Result<()> {
        let mut elements = array.elements.iter().peekable();

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

    /// Validates that `ObjectLiteral` covers `ObjectAssignment`.
    fn validate_object_literal_cover_assignment(&self, object: &LitObject) -> Result<()> {
        let mut props = object.props.iter().peekable();

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
