use crate::error::Result;
use crate::Parser;
use fajt_ast::{
    Expr, ExprMember, ExprOptionalCall, ExprOptionalMember, MemberObject, MemberProperty,
};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use fajt_lexer::{punct, LexerState};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses the `MemberExpression` goal symbol when you already know the left side.
    pub(super) fn parse_member_expr_right_side(
        &mut self,
        span_start: usize,
        left: MemberObject,
    ) -> Result<Expr> {
        let property = self.parse_member_property()?;
        let span = self.span_from(span_start);
        Ok(ExprMember {
            span,
            object: left,
            property,
        }
        .into())
    }

    /// Parses the `OptionalExpression` goal symbol. Left side must be known.
    pub(super) fn parse_optional_expr(&mut self, span_start: usize, left: Expr) -> Result<Expr> {
        let mut object = left;

        loop {
            match self.current() {
                token_matches!(ok: punct!("?.")) => {
                    if self.peek_matches(punct!("(")) {
                        object = self.parse_optional_call_expr(span_start, object)?;
                    } else {
                        object = self.parse_optional_member_expr(span_start, object)?;
                    }
                }
                token_matches!(ok: punct!(".") | punct!("[")) => {
                    object = self.parse_optional_member_expr(span_start, object)?;
                }
                token_matches!(ok: punct!("(")) => {
                    object = self.parse_optional_call_expr(span_start, object)?;
                }
                _ => break,
            }
        }

        Ok(object)
    }

    fn parse_optional_call_expr(&mut self, span_start: usize, callee: Expr) -> Result<Expr> {
        let optional = self.maybe_consume(punct!("?."))?;
        let (arguments_span, arguments) = self.parse_arguments()?;
        let span = self.span_from(span_start);

        Ok(ExprOptionalCall {
            span,
            callee: callee.into(),
            arguments_span,
            arguments,
            optional,
        }
        .into())
    }

    fn parse_optional_member_expr(&mut self, span_start: usize, object: Expr) -> Result<Expr> {
        let optional = self.current_matches(punct!("?."));
        let property = self.parse_optional_member_property()?;
        let span = self.span_from(span_start);

        Ok(ExprOptionalMember {
            span,
            object: object.into(),
            property,
            optional,
        }
        .into())
    }

    fn parse_optional_member_property(&mut self) -> Result<MemberProperty> {
        match self.current() {
            token_matches!(ok: punct!("?.")) if self.peek_matches(punct!("[")) => {
                self.consume()?;
                let property = self.parse_computed_property()?;
                Ok(MemberProperty::Expr(property.into()))
            }
            token_matches!(ok: punct!("[")) => {
                let property = self.parse_computed_property()?;
                Ok(MemberProperty::Expr(property.into()))
            }
            token_matches!(ok: punct!("?.") | punct!(".")) => {
                self.consume()?;
                let identifier = self.parse_identifier()?;
                Ok(MemberProperty::Ident(identifier))
            }
            _ => unreachable!(),
        }
    }

    fn parse_member_property(&mut self) -> Result<MemberProperty> {
        match self.current() {
            token_matches!(ok: punct!(".")) => {
                self.consume()?;
                let identifier = self.parse_identifier()?;
                Ok(MemberProperty::Ident(identifier))
            }
            token_matches!(ok: punct!("[")) => {
                let member = self.parse_computed_property()?;
                Ok(MemberProperty::Expr(member.into()))
            }
            _ => unreachable!(),
        }
    }

    fn parse_computed_property(&mut self) -> Result<Expr> {
        self.consume_assert(punct!("["))?;
        let expr = self.parse_expr()?;
        self.consume_assert(punct!("]"))?;

        Ok(expr)
    }
}
