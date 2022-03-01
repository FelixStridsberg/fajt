use crate::error::Result;
use crate::{Error, Parser};
use fajt_ast::{
    AssignmentOperator, Expr, ExprLiteral, Literal, Spanned,
};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::token::Token;
use fajt_lexer::LexerState;
use crate::static_semantics::{LitArraySemantics, LitObjectSemantics};

// TODO start thinking where to really place these.
impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Early error on invalid update expression argument.
    pub(super) fn validate_update_expression_argument(&self, argument: &Expr) -> Result<()> {
        if !self.is_assignment_target_type_simple(argument)? {
            return Err(Error::syntax_error(
                "Invalid update expression argument".to_owned(),
                argument.span().clone(),
            ));
        }

        Ok(())
    }

    /// Early error on invalid left hand side expression.
    pub(super) fn validate_left_side_expr(
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
