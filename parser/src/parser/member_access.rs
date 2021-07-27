use crate::ast::{
    Expression, MemberExpression, MemberObject, MemberProperty, OptionalCallExpression,
    OptionalMemberExpression,
};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::Result;
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    /// Parses the `MemberExpression` goal symbol when you already know the left side.
    pub(crate) fn parse_member_expression_right_side(
        &mut self,
        span_start: usize,
        left: MemberObject,
    ) -> Result<Expression> {
        let property = self.parse_member_property()?;
        let span = self.span_from(span_start);
        Ok(MemberExpression {
            span,
            object: left,
            property,
        }
        .into())
    }

    /// Parses the `OptionalExpression` goal symbol. Left side must be known.
    pub(crate) fn parse_optional_expression(
        &mut self,
        span_start: usize,
        left: Expression,
    ) -> Result<Expression> {
        let mut object = left;

        loop {
            match self.reader.current() {
                token_matches!(ok: punct!("?.")) => {
                    if token_matches!(self.reader.peek(), opt: punct!("(")) {
                        object = self.parse_optional_call_expression(span_start, object)?;
                    } else {
                        object = self.parse_optional_member_expression(span_start, object)?;
                    }
                }
                token_matches!(ok: punct!(".")) | token_matches!(ok: punct!("[")) => {
                    object = self.parse_optional_member_expression(span_start, object)?;
                }
                token_matches!(ok: punct!("(")) => {
                    object = self.parse_optional_call_expression(span_start, object)?;
                }
                _ => break,
            }
        }

        Ok(object)
    }

    fn parse_optional_call_expression(
        &mut self,
        span_start: usize,
        callee: Expression,
    ) -> Result<Expression> {
        let optional = token_matches!(self.reader.current(), ok: punct!("?."));
        if optional {
            self.reader.consume()?;
        }

        let (arguments_span, arguments) = self.parse_arguments()?;
        let span = self.span_from(span_start);

        Ok(OptionalCallExpression {
            span,
            callee,
            arguments_span,
            arguments,
            optional,
        }
        .into())
    }

    fn parse_optional_member_expression(
        &mut self,
        span_start: usize,
        object: Expression,
    ) -> Result<Expression> {
        let optional = token_matches!(self.reader.current(), ok: punct!("?."));
        let property = self.parse_optional_member_property()?;
        let span = self.span_from(span_start);

        Ok(OptionalMemberExpression {
            span,
            object,
            property,
            optional,
        }
        .into())
    }

    fn parse_optional_member_property(&mut self) -> Result<MemberProperty> {
        match self.reader.current() {
            token_matches!(ok: punct!("?."))
                if token_matches!(self.reader.peek(), opt: punct!("[")) =>
            {
                self.reader.consume()?;
                let property = self.parse_computed_property()?;
                Ok(MemberProperty::Expression(property))
            }
            token_matches!(ok: punct!("[")) => {
                let property = self.parse_computed_property()?;
                Ok(MemberProperty::Expression(property))
            }
            token_matches!(ok: punct!("?.")) | token_matches!(ok: punct!(".")) => {
                self.reader.consume()?;
                let identifier = self.parse_identifier()?;
                Ok(MemberProperty::Ident(identifier))
            }
            _ => unreachable!(),
        }
    }

    fn parse_member_property(&mut self) -> Result<MemberProperty> {
        match self.reader.current() {
            token_matches!(ok: punct!(".")) => {
                self.reader.consume()?;
                let identifier = self.parse_identifier()?;
                Ok(MemberProperty::Ident(identifier))
            }
            token_matches!(ok: punct!("[")) => {
                let member = self.parse_computed_property()?;
                Ok(MemberProperty::Expression(member))
            }
            _ => unreachable!(),
        }
    }

    fn parse_computed_property(&mut self) -> Result<Expression> {
        let open_bracket = self.reader.consume()?;
        debug_assert!(token_matches!(open_bracket, punct!("[")));

        let expression = self.parse_expression()?;
        let closing_bracket = self.reader.consume()?;
        if !token_matches!(closing_bracket, punct!("]")) {
            return err!(UnexpectedToken(closing_bracket));
        }

        Ok(expression)
    }
}
