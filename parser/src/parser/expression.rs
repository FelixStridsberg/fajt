use crate::ast::{
    Argument, AssignmentExpression, AwaitExpression, CallExpression, Callee, ConditionalExpression,
    Expression, Ident, Literal, MemberExpression, MemberObject, MemberProperty,
    MetaPropertyExpression, NewExpression, SequenceExpression, Super, ThisExpression,
    UnaryExpression, UpdateExpression, YieldExpression,
};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::Result;
use crate::Parser;

use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::{Span, Token};
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
        let span_start = self.position();
        match self.reader.current() {
            token_matches!(ok: keyword!("yield")) => self.parse_yield_expression(),
            token_matches!(ok: keyword!("async")) => self.parse_assignment_expression_async(),
            token_matches!(ok: punct!("(")) => self.parse_assignment_expression_open_parentheses(),
            _ if self.is_identifier()
                && token_matches!(self.reader.peek(), opt: punct!("=>"))
                && !self.reader.peek().unwrap().first_on_line =>
            {
                let span_start = self.position();
                let parameters = self.parse_arrow_identifier_argument()?;
                self.parse_arrow_function_expression(span_start, true, false, parameters)
            }
            _ => {
                let expression = self.parse_conditional_expression()?;

                // TODO assignment left side must be LeftHandSideExpression
                let assignment_operator = match self.reader.current() {
                    token_matches!(ok: punct!("=")) => Some(assignment_op!("=")),
                    token_matches!(ok: punct!("*=")) => Some(assignment_op!("*=")),
                    token_matches!(ok: punct!("/=")) => Some(assignment_op!("/=")),
                    token_matches!(ok: punct!("%=")) => Some(assignment_op!("%=")),
                    token_matches!(ok: punct!("+=")) => Some(assignment_op!("+=")),
                    token_matches!(ok: punct!("-=")) => Some(assignment_op!("-=")),
                    token_matches!(ok: punct!("<<=")) => Some(assignment_op!("<<=")),
                    token_matches!(ok: punct!(">>=")) => Some(assignment_op!(">>=")),
                    token_matches!(ok: punct!(">>>=")) => Some(assignment_op!(">>>=")),
                    token_matches!(ok: punct!("&=")) => Some(assignment_op!("&=")),
                    token_matches!(ok: punct!("^=")) => Some(assignment_op!("^=")),
                    token_matches!(ok: punct!("|=")) => Some(assignment_op!("|=")),
                    token_matches!(ok: punct!("**=")) => Some(assignment_op!("**=")),
                    _ => None,
                };

                if let Some(operator) = assignment_operator {
                    self.reader.consume()?; // consume the operator
                    let right = self.parse_assignment_expression()?;
                    let span = self.span_from(span_start);
                    Ok(AssignmentExpression {
                        span,
                        operator,
                        left: expression,
                        right,
                    }
                    .into())
                } else {
                    Ok(expression)
                }
            }
        }
    }

    /// Parses the part of `AssignmentExpression` that start with the `(` punctuator.
    fn parse_assignment_expression_open_parentheses(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let parenthesized_or_arrow_parameters =
            self.parse_cover_parenthesized_and_arrow_parameters()?;

        if token_matches!(self.reader.current(), ok: punct!("=>"))
            && !self.reader.current().unwrap().first_on_line
        {
            let parameters = parenthesized_or_arrow_parameters.into_arrow_parameters()?;
            self.parse_arrow_function_expression(span_start, false, false, parameters)
        } else {
            parenthesized_or_arrow_parameters.into_expression()
        }
    }

    /// Parses the part of `AssignmentExpression` that starts with the `async` keyword.
    /// Note that most of the complexity comes because of the ambiguity between:
    /// 1. `async a => {}` // Async arrow function without parameter parentheses.
    /// 2. `async(a) => {}` // Async arrow function with parentheses.
    /// 3. `async(a)` // Function call where `async` is an identifier and not a keyword.
    fn parse_assignment_expression_async(&mut self) -> Result<Expression> {
        if self.peek_is_identifier() {
            let span_start = self.position();
            self.reader.consume()?;
            let parameters = self.parse_arrow_identifier_argument()?;
            return self.parse_arrow_function_expression(span_start, true, true, parameters);
        }

        if token_matches!(self.reader.peek(), opt: punct!("(")) {
            let span_start = self.position();
            let call_or_arrow_parameters = self.parse_cover_call_and_async_arrow_head()?;
            if token_matches!(self.reader.current(), ok: punct!("=>")) {
                let parameters = call_or_arrow_parameters.into_arrow_parameters()?;
                self.parse_arrow_function_expression(span_start, false, true, parameters)
            } else {
                call_or_arrow_parameters.into_call()
            }
        } else {
            self.parse_conditional_expression()
        }
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
        let argument = self.parse_left_hand_side_expression(false)?;
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
    /// TODO `only_call` don't make sense, hide between function call
    pub(super) fn parse_left_hand_side_expression(
        &mut self,
        only_call: bool,
    ) -> Result<Expression> {
        match self.reader.current() {
            token_matches!(ok: keyword!("super"))
                if token_matches!(self.reader.peek(), opt: punct!("(")) =>
            {
                self.parse_super_call_expression()
            }
            token_matches!(ok: keyword!("import"))
                if token_matches!(self.reader.peek(), opt: punct!("(")) =>
            {
                self.parse_import_call_expression()
            }
            token_matches!(ok: keyword!("new"))
                if !token_matches!(self.reader.peek(), opt: punct!(".")) && !only_call =>
            {
                self.parse_new_expression()
            }
            _ => {
                let span_start = self.position();
                let mut expression = self.parse_member_expression()?;

                loop {
                    match self.reader.current() {
                        token_matches!(ok: punct!("(")) => {
                            let (arguments_span, arguments) = self.parse_arguments()?;
                            let span = self.span_from(span_start);
                            expression = CallExpression {
                                span,
                                callee: Callee::Expression(expression),
                                arguments_span,
                                arguments,
                            }
                            .into();
                        }
                        token_matches!(ok: punct!(".")) => {
                            expression = self.parse_member_property(
                                span_start,
                                MemberObject::Expression(expression),
                            )?;
                        }
                        _ => break,
                    }
                }

                Ok(expression)
            }
        }

        // TODO CallExpression
        // TODO OptionalExpression
    }

    /// Parses the `SuperCall` goal symbol.
    fn parse_super_call_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        self.reader.consume()?;
        let (arguments_span, arguments) = self.parse_arguments()?;
        let span = self.span_from(span_start);
        Ok(CallExpression {
            span,
            callee: Callee::Super,
            arguments_span,
            arguments,
        }
        .into())
    }

    /// Parses the `ImportCall` goal symbol.
    fn parse_import_call_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        self.reader.consume()?;

        let (arguments_span, arguments) = self.parse_import_argument()?;

        let span = self.span_from(span_start);
        Ok(CallExpression {
            span,
            callee: Callee::Import,
            arguments_span,
            arguments,
        }
        .into())
    }

    /// Parses the `NewExpression` goal symbol.
    fn parse_new_expression(&mut self) -> Result<Expression> {
        if token_matches!(self.reader.current(), ok: keyword!("new"))
            && !token_matches!(self.reader.peek(), opt: punct!("."))
        {
            let span_start = self.position();
            self.reader.consume()?;

            let callee = self.parse_new_expression()?;

            let (arguments_span, arguments) =
                if token_matches!(self.reader.current(), ok: punct!("(")) {
                    self.parse_arguments()
                        .map(|(span, args)| (Some(span), args))?
                } else {
                    (None, Vec::new())
                };

            let span = self.span_from(span_start);
            Ok(NewExpression {
                span,
                callee,
                arguments_span,
                arguments,
            }
            .into())
        } else {
            self.parse_member_expression()
        }
    }

    fn parse_arguments(&mut self) -> Result<(Span, Vec<Argument>)> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, punct!("(")));

        let mut arguments = Vec::new();

        loop {
            match self.reader.current() {
                token_matches!(ok: punct!(")")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(ok: punct!("...")) => {
                    self.reader.consume()?;
                    arguments.push(Argument::Spread(self.parse_assignment_expression()?));
                    self.consume_parameter_delimiter()?;
                }
                _ => {
                    arguments.push(Argument::Expression(self.parse_assignment_expression()?));
                    self.consume_parameter_delimiter()?;
                }
            }
        }

        let span = self.span_from(span_start);
        Ok((span, arguments))
    }

    fn parse_import_argument(&mut self) -> Result<(Span, Vec<Argument>)> {
        let span_start = self.position();
        let open_parentheses = self.reader.consume()?;
        if !token_matches!(open_parentheses, punct!("(")) {
            return err!(UnexpectedToken(open_parentheses));
        }

        let expression = self.parse_assignment_expression()?;

        let close_parentheses = self.reader.consume()?;
        if !token_matches!(close_parentheses, punct!(")")) {
            return err!(UnexpectedToken(open_parentheses));
        }

        let span = self.span_from(span_start);
        Ok((span, vec![Argument::Expression(expression)]))
    }

    /// Parses the `MemberExpression` goal symbol.
    /// NOTE: The `new MemberExpression Arguments` is parsed in `NewExpression`
    pub fn parse_member_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let mut expression = self.parse_terminal_member_expression_or_primary()?;

        loop {
            if token_matches!(self.reader.current(), ok: punct!(".") | punct!("[")) {
                expression =
                    self.parse_member_property(span_start, MemberObject::Expression(expression))?;
            } else {
                break;
            }
        }

        Ok(expression)

        // TODO MemberExpression TemplateLiteral
    }

    /// Parses the `MemberExpressions` that starts with a terminal, or `PrimaryExpression` goal symbols.
    fn parse_terminal_member_expression_or_primary(&mut self) -> Result<Expression> {
        let peek = self.reader.peek();
        match self.reader.current()? {
            token_matches!(keyword!("super")) => {
                let span_start = self.position();
                let super_token = self.reader.consume()?;

                if !token_matches!(self.reader.current(), ok: punct!(".") | punct!("[")) {
                    todo!("Error, unexpected super")
                }

                self.parse_member_property(
                    span_start,
                    MemberObject::Super(Super {
                        span: super_token.span,
                    }),
                )
            }
            token_matches!(keyword!("new")) if token_matches!(peek, opt: punct!(".")) => {
                let span_start = self.position();
                let new_token = self.reader.consume()?;
                self.reader.consume()?; // .

                let property = self.parse_identifier()?;
                if property.name != "target" {
                    todo!("Error, expected `target` after `new.`");
                }

                let span = self.span_from(span_start);
                Ok(MetaPropertyExpression {
                    span,
                    meta: Ident::new("new", new_token.span),
                    property,
                }
                .into())
            }
            token_matches!(keyword!("import")) if token_matches!(peek, opt: punct!(".")) => {
                let span_start = self.position();
                let import_token = self.reader.consume()?;
                self.reader.consume()?; // .

                let property = self.parse_identifier()?;
                if property.name != "meta" {
                    todo!("Error, expected `meta` after `import.`");
                }

                let span = self.span_from(span_start);
                Ok(MetaPropertyExpression {
                    span,
                    meta: Ident::new("import", import_token.span),
                    property,
                }
                .into())
            }
            _ => self.parse_primary_expression(),
        }
    }

    fn parse_member_property(
        &mut self,
        span_start: usize,
        object: MemberObject,
    ) -> Result<Expression> {
        match self.reader.current() {
            token_matches!(ok: punct!(".")) => {
                self.reader.consume()?;
                let identifier = self.parse_identifier()?;
                let span = self.span_from(span_start);
                Ok(MemberExpression {
                    span,
                    object,
                    property: MemberProperty::Ident(identifier),
                }
                .into())
            }
            token_matches!(ok: punct!("[")) => {
                self.reader.consume()?;
                let member = self.parse_expression()?;
                let closing_bracket = self.reader.consume()?;
                if !token_matches!(closing_bracket, punct!("]")) {
                    return err!(UnexpectedToken(closing_bracket));
                }
                let span = self.span_from(span_start);
                Ok(MemberExpression {
                    span,
                    object,
                    property: MemberProperty::Expression(member),
                }
                .into())
            }
            _ => unreachable!(),
        }
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
