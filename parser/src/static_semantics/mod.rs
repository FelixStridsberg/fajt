use crate::error::Result;
use crate::{Context, Error};
use fajt_ast::{AssignmentOperator, Expr, ExprLiteral, Literal, Spanned};

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
                    literal: Literal::Array(_),
                    ..
                }) => {
                    todo!("Array literal");
                    //return;
                }
                Expr::Literal(ExprLiteral {
                    literal: Literal::Object(_),
                    ..
                }) => {
                    todo!("Object literal");
                    //return;
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
}
