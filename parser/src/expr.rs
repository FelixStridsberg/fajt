use crate::error::ErrorKind::{SyntaxError, UnexpectedToken};
use crate::error::{Result, ThenTry};
use crate::{ContextModify, Parser};
use fajt_ast::assignment_op;
use fajt_ast::unary_op;
use fajt_ast::update_op;
use fajt_ast::{
    Argument, Callee, Expr, ExprAssignment, ExprAwait, ExprCall, ExprConditional, ExprMetaProperty,
    ExprNew, ExprSequence, ExprThis, ExprUnary, ExprUpdate, ExprYield, Ident, Literal,
    MemberObject, Span, Super,
};
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
    pub(super) fn parse_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let expr = self.parse_assignment_expr()?;

        if self.current_matches(punct!(",")) {
            return self.parse_expr_sequence(span_start, expr);
        }

        Ok(expr)
    }

    /// Continues parsing of `Expression` if the current expression is a sequence.
    fn parse_expr_sequence(&mut self, span_start: usize, initial_expr: Expr) -> Result<Expr> {
        let mut expr = Vec::with_capacity(5);
        expr.push(initial_expr);

        loop {
            if self.current_matches(punct!(",")) {
                self.consume()?;
                expr.push(self.parse_assignment_expr()?);
            } else {
                break;
            }
        }

        let span = self.span_from(span_start);
        Ok(ExprSequence { span, expr }.into())
    }

    /// Parses the `AssignmentExpression` goal symbol.
    pub(super) fn parse_assignment_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        match self.current() {
            token_matches!(ok: keyword!("yield")) if self.context.is_yield => {
                self.parse_yield_expr()
            }
            token_matches!(ok: keyword!("async")) => self.parse_assignment_expr_async(),
            token_matches!(ok: punct!("(")) => self.parse_assignment_expr_open_parentheses(),
            _ if self.is_identifier()
                && !self.followed_by_new_lined()
                && self.peek_matches(punct!("=>")) =>
            {
                let span_start = self.position();
                let parameters = self.parse_arrow_identifier_argument()?;
                self.parse_arrow_function_expr(span_start, true, parameters)
            }
            _ => {
                let expr = self.parse_conditional_expr()?;

                // TODO assignment left side must be LeftHandSideExpression
                let assignment_operator = match self.current() {
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
                    self.consume()?; // consume the operator
                    let right = self.parse_assignment_expr()?;
                    let span = self.span_from(span_start);
                    Ok(ExprAssignment {
                        span,
                        operator,
                        left: expr.into(),
                        right: right.into(),
                    }
                    .into())
                } else {
                    Ok(expr)
                }
            }
        }
    }

    /// Parses the part of `AssignmentExpression` that start with the `(` punctuator.
    fn parse_assignment_expr_open_parentheses(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let parenthesized_or_arrow_parameters =
            self.parse_cover_parenthesized_and_arrow_parameters()?;

        if self.current_matches(punct!("=>")) && !self.current().unwrap().first_on_line {
            let parameters = parenthesized_or_arrow_parameters.into_arrow_parameters()?;
            self.parse_arrow_function_expr(span_start, false, parameters)
        } else {
            parenthesized_or_arrow_parameters.into_expr()
        }
    }

    /// Parses the part of `AssignmentExpression` that starts with the `async` keyword.
    /// Note that most of the complexity comes because of the ambiguity between:
    /// 1. `async => {}` // Non async arrow function with the name async.
    /// 2. `async a => {}` // Async arrow function without parameter parentheses.
    /// 3. `async(a) => {}` // Async arrow function with parentheses.
    /// 4. `async(a)` // Function call where `async` is an identifier and not a keyword.
    fn parse_assignment_expr_async(&mut self) -> Result<Expr> {
        match self.peek() {
            token_matches!(opt: punct!("=>")) => {
                let span_start = self.position();
                let parameters = self.parse_arrow_identifier_argument()?;
                self.parse_arrow_function_expr(span_start, true, parameters)
            }
            token_matches!(opt: punct!("(")) => {
                let span_start = self.position();
                let call_or_arrow_parameters = self.parse_cover_call_and_async_arrow_head()?;
                if self.current_matches(punct!("=>")) {
                    let parameters = call_or_arrow_parameters.into_arrow_parameters()?;
                    self.parse_async_arrow_function_expr(span_start, false, parameters)
                } else {
                    call_or_arrow_parameters.into_call()
                }
            }
            _ if self.peek_is_identifier() => {
                let span_start = self.position();
                self.consume()?;
                let parameters = self.parse_arrow_identifier_argument()?;
                self.parse_async_arrow_function_expr(span_start, true, parameters)
            }
            _ => self.parse_conditional_expr(),
        }
    }

    /// Parses the `YieldExpression` goal symbol.
    fn parse_yield_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(keyword!("yield"))?;

        if self.is_end() {
            let span = self.span_from(span_start);
            return Ok(ExprYield {
                span,
                argument: None,
                delegate: false,
            }
            .into());
        }

        let next_token = self.current()?;
        let has_argument = !next_token.first_on_line && !token_matches!(next_token, punct!(";"));
        let delegate = has_argument && token_matches!(next_token, punct!("*"));
        if delegate {
            self.consume()?;
        }

        let argument = has_argument.then_try(|| Ok(Box::new(self.parse_assignment_expr()?)))?;
        let span = self.span_from(span_start);
        Ok(ExprYield {
            span,
            argument,
            delegate,
        }
        .into())
    }

    /// Parses the `ConditionalExpression` goal symbol.
    fn parse_conditional_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let expr = self.parse_short_circuit_expr()?;

        if self.current_matches(punct!("?")) {
            self.consume()?;
            let consequent = self.parse_assignment_expr()?;

            self.consume_assert(punct!(":"))?;

            let alternate = self.parse_assignment_expr()?;
            let span = self.span_from(span_start);
            Ok(ExprConditional {
                span,
                condition: expr.into(),
                consequent: consequent.into(),
                alternate: alternate.into(),
            }
            .into())
        } else {
            Ok(expr)
        }
    }

    /// Parses the `UnaryExpression` goal symbol.
    pub(super) fn parse_unary_expr(&mut self) -> Result<Expr> {
        let operator = match self.current()? {
            token_matches!(punct!("+")) => Some(unary_op!("+")),
            token_matches!(punct!("-")) => Some(unary_op!("-")),
            token_matches!(punct!("~")) => Some(unary_op!("~")),
            token_matches!(punct!("!")) => Some(unary_op!("!")),
            token_matches!(keyword!("delete")) => Some(unary_op!("delete")),
            token_matches!(keyword!("void")) => Some(unary_op!("void")),
            token_matches!(keyword!("typeof")) => Some(unary_op!("typeof")),
            token_matches!(keyword!("await")) if self.context.is_await => {
                let span_start = self.position();
                self.consume()?;
                let argument = self.parse_unary_expr()?.into();
                let span = self.span_from(span_start);
                return Ok(ExprAwait { span, argument }.into());
            }
            _ => None,
        };

        if let Some(operator) = operator {
            let span_start = self.position();
            self.consume()?;
            let argument = self.parse_unary_expr()?.into();
            let span = self.span_from(span_start);
            Ok(ExprUnary {
                span,
                operator,
                argument,
            }
            .into())
        } else {
            self.parse_update_expr()
        }
    }

    /// Parses the `UpdateExpression` goal symbol.
    fn parse_update_expr(&mut self) -> Result<Expr> {
        let prefix_operator = match self.current()? {
            token_matches!(punct!("++")) => Some(update_op!("++")),
            token_matches!(punct!("--")) => Some(update_op!("--")),
            _ => None,
        };

        if let Some(operator) = prefix_operator {
            let span_start = self.position();
            self.consume()?;
            let argument = self.parse_unary_expr()?.into();
            let span = self.span_from(span_start);
            return Ok(ExprUpdate {
                span,
                operator,
                prefix: true,
                argument,
            }
            .into());
        }

        let span_start = self.position();
        let argument = self.parse_left_hand_side_expr()?;
        let suffix_operator = match self.current() {
            token_matches!(ok: punct!("++")) => Some(update_op!("++")),
            token_matches!(ok: punct!("--")) => Some(update_op!("--")),
            _ => None,
        };

        if let Some(operator) = suffix_operator {
            if self.current()?.first_on_line {
                return Ok(argument);
            }

            self.consume()?;
            let span = self.span_from(span_start);
            Ok(ExprUpdate {
                span,
                operator,
                prefix: false,
                argument: argument.into(),
            }
            .into())
        } else {
            Ok(argument)
        }
    }

    /// Parses the `LeftHandSideExpression` goal symbol.
    pub(super) fn parse_left_hand_side_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let expr = match self.current() {
            token_matches!(ok: keyword!("new")) if !self.peek_matches(punct!(".")) => {
                self.parse_new_expr()
            }
            token_matches!(ok: keyword!("super")) if self.peek_matches(punct!("(")) => {
                self.parse_super_call_expr()
            }
            token_matches!(ok: keyword!("import")) if self.peek_matches(punct!("(")) => {
                self.parse_import_call_expr()
            }
            _ => {
                let span_start = self.position();
                let mut expr = self.parse_member_expr()?;

                loop {
                    match self.current() {
                        token_matches!(ok: punct!("(")) => {
                            let (arguments_span, arguments) = self.parse_arguments()?;
                            let span = self.span_from(span_start);
                            expr = ExprCall {
                                span,
                                callee: Callee::Expr(expr.into()),
                                arguments_span,
                                arguments,
                            }
                            .into();
                        }
                        token_matches!(ok: punct!(".") | punct!("[")) => {
                            expr = self.parse_member_expr_right_side(
                                span_start,
                                MemberObject::Expr(expr.into()),
                            )?;
                        }
                        // TODO CallExpression TemplateLiteral
                        _ => break,
                    }
                }

                Ok(expr)
            }
        }?;

        if self.current_matches(punct!("?.")) {
            // The NewExpression goal symbol handles nested news and is not included in the
            // OptionalExpression goal symbol as a base for the chain.
            if expr.is_nested_new() {
                return err!(SyntaxError(
                    "Invalid optional chain from new expression".to_owned(),
                    self.current().unwrap().span.clone()
                ));
            }

            self.parse_optional_expr(span_start, expr)
        } else {
            Ok(expr)
        }
    }

    /// Parses the `SuperCall` goal symbol.
    fn parse_super_call_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume()?;
        let (arguments_span, arguments) = self.parse_arguments()?;
        let span = self.span_from(span_start);
        Ok(ExprCall {
            span,
            callee: Callee::Super,
            arguments_span,
            arguments,
        }
        .into())
    }

    /// Parses the `ImportCall` goal symbol.
    fn parse_import_call_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume()?;

        let (arguments_span, arguments) = self.parse_import_argument()?;

        let span = self.span_from(span_start);
        Ok(ExprCall {
            span,
            callee: Callee::Import,
            arguments_span,
            arguments,
        }
        .into())
    }

    fn parse_import_argument(&mut self) -> Result<(Span, Vec<Argument>)> {
        let span_start = self.position();

        self.consume_assert(punct!("("))?;
        let expr = self.parse_assignment_expr()?;
        self.consume_assert(punct!(")"))?;

        let span = self.span_from(span_start);
        Ok((span, vec![Argument::Expr(expr)]))
    }

    /// Parses the `NewExpression` goal symbol.
    fn parse_new_expr(&mut self) -> Result<Expr> {
        if self.current_matches(keyword!("new")) && !self.current_matches(punct!(".")) {
            let span_start = self.position();
            self.consume()?;

            let callee = self.parse_new_expr()?.into();

            let (arguments_span, arguments) = if self.current_matches(punct!("(")) {
                self.parse_arguments()
                    .map(|(span, args)| (Some(span), args))?
            } else {
                (None, Vec::new())
            };

            let span = self.span_from(span_start);
            Ok(ExprNew {
                span,
                callee,
                arguments_span,
                arguments,
            }
            .into())
        } else {
            self.parse_member_expr()
        }
    }

    /// Parses the `Arguments` goal symbol.
    pub(super) fn parse_arguments(&mut self) -> Result<(Span, Vec<Argument>)> {
        let span_start = self.position();
        self.consume_assert(punct!("("))?;

        let mut arguments = Vec::new();

        loop {
            match self.current() {
                token_matches!(ok: punct!(")")) => {
                    self.consume()?;
                    break;
                }
                token_matches!(ok: punct!("...")) => {
                    self.consume()?;
                    arguments.push(Argument::Spread(self.parse_assignment_expr()?));
                    self.consume_parameter_delimiter()?;
                }
                _ => {
                    arguments.push(Argument::Expr(self.parse_assignment_expr()?));
                    self.consume_parameter_delimiter()?;
                }
            }
        }

        let span = self.span_from(span_start);
        Ok((span, arguments))
    }

    /// Parses the `MemberExpression` goal symbol.
    /// NOTE: The `new MemberExpression Arguments` is parsed in `NewExpression`
    pub fn parse_member_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let mut expr = self.parse_member_expr_non_recursive()?;

        loop {
            if token_matches!(self.current(), ok: punct!(".") | punct!("[")) {
                expr =
                    self.parse_member_expr_right_side(span_start, MemberObject::Expr(expr.into()))?;
            } else {
                break;
            }
        }

        Ok(expr)

        // TODO MemberExpression TemplateLiteral
    }

    /// Parses the non recursive parts of `MemberExpressions`.
    fn parse_member_expr_non_recursive(&mut self) -> Result<Expr> {
        match self.current()? {
            token_matches!(keyword!("super")) => {
                let span_start = self.position();
                let super_token = self.consume()?;

                if !token_matches!(self.current(), ok: punct!(".") | punct!("[")) {
                    todo!("Error, unexpected super")
                }

                self.parse_member_expr_right_side(
                    span_start,
                    MemberObject::Super(Super {
                        span: super_token.span,
                    }),
                )
            }
            token_matches!(keyword!("new")) if self.peek_matches(punct!(".")) => {
                let span_start = self.position();
                let new_token = self.consume()?;
                self.consume()?; // .

                let property = self.parse_identifier()?;
                if property.name != "target" {
                    todo!("Error, expected `target` after `new.`");
                }

                let span = self.span_from(span_start);
                Ok(ExprMetaProperty {
                    span,
                    meta: Ident::new("new", new_token.span),
                    property,
                }
                .into())
            }
            token_matches!(keyword!("import")) if self.peek_matches(punct!(".")) => {
                let span_start = self.position();
                let import_token = self.consume()?;
                self.consume()?; // .

                let property = self.parse_identifier()?;
                if property.name != "meta" {
                    todo!("Error, expected `meta` after `import.`");
                }

                let span = self.span_from(span_start);
                Ok(ExprMetaProperty {
                    span,
                    meta: Ident::new("import", import_token.span),
                    property,
                }
                .into())
            }
            _ => self.parse_primary_expr(),
        }
    }

    /// Parses the `PrimaryExpression` goal symbol.
    fn parse_primary_expr(&mut self) -> Result<Expr> {
        Ok(match self.current()? {
            token_matches!(keyword!("this")) => self.parse_this_expr()?,
            token_matches!(keyword!("null")) => self.consume_literal(Literal::Null)?,
            token_matches!(keyword!("true")) => self.consume_literal(Literal::Boolean(true))?,
            token_matches!(keyword!("false")) => self.consume_literal(Literal::Boolean(false))?,
            token_matches!(@literal) => self.parse_literal()?,
            token_matches!(punct!("[")) => self.parse_array_literal()?,
            token_matches!(punct!("{")) => self.parse_object_literal()?,
            token_matches!(keyword!("function")) => self.parse_function_expr()?,
            token_matches!(keyword!("class")) => self
                .with_context(ContextModify::new().set_strict(true))
                .parse_class_expr()?,
            token_matches!(keyword!("async")) if !self.followed_by_new_lined() => {
                self.parse_async_function_expr()?
            }
            token_matches!(punct!("/")) => todo!("RegularExpressionLiteral"),
            // token_matches!(punct!("`")) => todo!("TemplateLiteral"), TODO missing from lexer
            token_matches!(punct!("(")) => self
                .parse_cover_parenthesized_and_arrow_parameters()?
                .into_expr()?,
            _ if self.is_identifier() => self.parse_identifier_reference()?,
            _ => return err!(UnexpectedToken(self.consume()?)),
        })
    }

    /// Parses the `this` expression which is part of the `PrimaryExpression` goal symbol.
    fn parse_this_expr(&mut self) -> Result<Expr> {
        let token = self.consume_assert(keyword!("this"))?;
        Ok(ExprThis::new(token.span).into())
    }

    /// Parses the `IdentifierReference` goal symbol.
    fn parse_identifier_reference(&mut self) -> Result<Expr> {
        let ident = self.parse_identifier()?;
        Ok(ident.into())
    }

    /// Parses the `Initializer` goal symbol.
    pub(super) fn parse_initializer(&mut self) -> Result<Expr> {
        self.consume()?; // Skip =
        self.parse_assignment_expr()
    }
}
