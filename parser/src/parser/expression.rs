use crate::ast::{
    AwaitExpression, ConditionalExpression, Expression,
    Literal, SequenceExpression, ThisExpression, UnaryExpression, UpdateExpression,
    YieldExpression,
};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::Result;
use crate::Parser;

use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    /// Parses the `Expression` goal symbol.
    pub(crate) fn parse_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let expression = self.parse_assignment_expression()?;

        if token_matches!(self.reader.current(), ok: punct!(",")) {
            return self.parse_expression_sequence(span_start, expression);
        }

        Ok(expression)
    }

    /// Continues parsing of `Expression` if the current expression is a sequence.
    fn parse_expression_sequence(
        &mut self,
        span_start: usize,
        initial_expression: Expression,
    ) -> Result<Expression> {
        let mut expressions = Vec::with_capacity(5);
        expressions.push(initial_expression);

        loop {
            if token_matches!(self.reader.current(), ok: punct!(",")) {
                self.reader.consume()?;
                expressions.push(self.parse_assignment_expression()?);
            } else {
                break;
            }
        }

        let span = self.span_from(span_start);
        Ok(SequenceExpression { span, expressions }.into())
    }

    /// Parses the `AssignmentExpression` goal symbol.
    pub(super) fn parse_assignment_expression(&mut self) -> Result<Expression> {
        match self.reader.current() {
            token_matches!(ok: keyword!("yield")) => self.parse_yield_expression(),
            token_matches!(ok: keyword!("async")) if self.followed_by_new_lined() => {
                todo!()
            }
            token_matches!(ok: punct!("(")) => {
                let span_start = self.position();
                let parenthesized_or_arrow_parameters =
                    self.parse_cover_parenthesized_and_arrow_parameters()?;

                if token_matches!(self.reader.current(), ok: punct!("=>"))
                    && !self.reader.current().unwrap().first_on_line
                {
                    let parameters = parenthesized_or_arrow_parameters.into_arrow_parameters()?;
                    self.parse_arrow_function_expression(span_start, false, parameters)
                } else {
                    parenthesized_or_arrow_parameters.into_expression()
                }
            }
            _ if self.is_identifier()
                && token_matches!(self.reader.peek(), opt: punct!("=>"))
                && !self.reader.peek().unwrap().first_on_line =>
            {
                let span_start = self.position();
                let parameters = Some(self.parse_arrow_identifier_argument()?);
                self.parse_arrow_function_expression(span_start, true, parameters)
            }
            _ => self.parse_conditional_expression(),
        }

        // TODO AsyncArrowFunction
        // TODO LeftHandSideExpression
    }

    /// Parses the `YieldExpression` goal symbol.
    fn parse_yield_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let yield_token = self.reader.consume()?;
        debug_assert!(token_matches!(yield_token, keyword!("yield")));

        if self.reader.is_end() {
            let span = self.span_from(span_start);
            return Ok(YieldExpression {
                span,
                argument: None,
                delegate: false,
            }
            .into());
        }

        let next_token = self.reader.current()?;
        let has_argument = !next_token.first_on_line;
        let delegate = has_argument && token_matches!(next_token, punct!("*"));
        if delegate {
            self.reader.consume()?;
        }

        let argument = if has_argument {
            Some(self.parse_assignment_expression()?)
        } else {
            None
        };
        let span = self.span_from(span_start);
        Ok(YieldExpression {
            span,
            argument,
            delegate,
        }
        .into())
    }

    /// Parses the `ConditionalExpression` goal symbol.
    fn parse_conditional_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let expression = self.parse_short_circuit_expression()?;

        if token_matches!(self.reader.current(), ok: punct!("?")) {
            self.reader.consume()?;
            let consequent = self.parse_assignment_expression()?;

            let token = self.reader.consume()?;
            if !token_matches!(token, punct!(":")) {
                return err!(UnexpectedToken(token));
            }

            let alternate = self.parse_assignment_expression()?;
            let span = self.span_from(span_start);
            Ok(ConditionalExpression {
                span,
                condition: expression,
                consequent,
                alternate,
            }
            .into())
        } else {
            Ok(expression)
        }
    }

    /// Parses the `UnaryExpression` goal symbol.
    pub(super) fn parse_unary_expression(&mut self) -> Result<Expression> {
        let operator = match self.reader.current()? {
            token_matches!(punct!("+")) => Some(unary_op!("+")),
            token_matches!(punct!("-")) => Some(unary_op!("-")),
            token_matches!(punct!("~")) => Some(unary_op!("~")),
            token_matches!(punct!("!")) => Some(unary_op!("!")),
            token_matches!(keyword!("delete")) => Some(unary_op!("delete")),
            token_matches!(keyword!("void")) => Some(unary_op!("void")),
            token_matches!(keyword!("typeof")) => Some(unary_op!("typeof")),
            token_matches!(keyword!("await")) if self.context.is_await => {
                let span_start = self.position();
                self.reader.consume()?;
                let argument = self.parse_unary_expression()?;
                let span = self.span_from(span_start);
                return Ok(AwaitExpression { span, argument }.into());
            }
            _ => None,
        };

        if let Some(operator) = operator {
            let span_start = self.position();
            self.reader.consume()?;
            let argument = self.parse_unary_expression()?;
            let span = self.span_from(span_start);
            Ok(UnaryExpression {
                span,
                operator,
                argument,
            }
            .into())
        } else {
            self.parse_update_expression()
        }
    }

    /// Parses the `UpdateExpression` goal symbol.
    fn parse_update_expression(&mut self) -> Result<Expression> {
        let prefix_operator = match self.reader.current()? {
            token_matches!(punct!("++")) => Some(update_op!("++")),
            token_matches!(punct!("--")) => Some(update_op!("--")),
            _ => None,
        };

        if let Some(operator) = prefix_operator {
            let span_start = self.position();
            self.reader.consume()?;
            let argument = self.parse_unary_expression()?;
            let span = self.span_from(span_start);
            return Ok(UpdateExpression {
                span,
                operator,
                prefix: true,
                argument,
            }
            .into());
        }

        let span_start = self.position();
        let argument = self.parse_left_hand_side_expression()?;
        let suffix_operator = match self.reader.current() {
            token_matches!(ok: punct!("++")) => Some(update_op!("++")),
            token_matches!(ok: punct!("--")) => Some(update_op!("--")),
            _ => None,
        };

        if let Some(operator) = suffix_operator {
            if self.reader.current()?.first_on_line {
                return Ok(argument);
            }

            self.reader.consume()?;
            let span = self.span_from(span_start);
            Ok(UpdateExpression {
                span,
                operator,
                prefix: false,
                argument,
            }
            .into())
        } else {
            Ok(argument)
        }
    }

    /// Parses the `LeftHandSideExpression` goal symbol.
    pub(super) fn parse_left_hand_side_expression(&mut self) -> Result<Expression> {
        self.parse_new_expression()
        // TODO CallExpression
        // TODO OptionalExpression
    }

    /// Parses the `NewExpression` goal symbol.
    fn parse_new_expression(&mut self) -> Result<Expression> {
        if token_matches!(self.reader.current(), ok: keyword!("new")) {
            todo!("NewExpression")
        } else {
            self.parse_member_expression()
        }
    }

    /// Parses the `MemberExpression` goal symbol.
    fn parse_member_expression(&mut self) -> Result<Expression> {
        self.parse_primary_expression()
        // TODO MemberExpression [ Expression ]
        // TODO MemberExpression . IdentifierName
        // TODO MemberExpression TemplateLiteral
        // TODO SuperProperty
        // TODO MetaProperty
        // TODO new MemberExpression Arguments
    }

    /// Parses the `PrimaryExpression` goal symbol.
    fn parse_primary_expression(&mut self) -> Result<Expression> {
        Ok(match self.reader.current()? {
            token_matches!(keyword!("this")) => self.parse_this_expression()?,
            token_matches!(keyword!("null")) => self.consume_literal(Literal::Null)?,
            token_matches!(keyword!("true")) => self.consume_literal(Literal::Boolean(true))?,
            token_matches!(keyword!("false")) => self.consume_literal(Literal::Boolean(false))?,
            token_matches!(@literal) => self.parse_literal()?,
            token_matches!(punct!("[")) => self.parse_array_literal()?,
            token_matches!(punct!("{")) => self.parse_object_literal()?,
            token_matches!(keyword!("function")) => self.parse_function_expression()?,
            token_matches!(keyword!("class")) => self.parse_class_expression()?,
            token_matches!(keyword!("async")) if !self.followed_by_new_lined() => {
                self.parse_async_function_expression()?
            }
            token_matches!(punct!("/")) => todo!("RegularExpressionLiteral"),
            // token_matches!(punct!("`")) => todo!("TemplateLiteral"), TODO missing from lexer
            token_matches!(punct!("(")) => self
                .parse_cover_parenthesized_and_arrow_parameters()?
                .into_expression()?,
            _ if self.is_identifier() => self.parse_identifier_reference()?,
            r => unimplemented!("TOKEN: {:?}", r),
        })
    }

    /// Parses the `this` expression which is part of the `PrimaryExpression` goal symbol.
    fn parse_this_expression(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("this")));

        Ok(ThisExpression::new(token.span).into())
    }

    /// Parses the `IdentifierReference` goal symbol.
    fn parse_identifier_reference(&mut self) -> Result<Expression> {
        let ident = self.parse_identifier()?;

        // Consume potential trailing semi colon TODO how to handle these in general?
        if token_matches!(self.reader.current(), ok: punct!(";")) {
            self.reader.consume()?;
        }

        Ok(ident.into())
    }

    /// Parses the `Initializer` goal symbol.
    pub(super) fn parse_initializer(&mut self) -> Result<Expression> {
        self.reader.consume()?; // Skip =
        self.parse_assignment_expression()
    }
}
