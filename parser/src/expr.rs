use crate::error::Result;
use crate::{Context, Error, Parser};
use fajt_ast::{assignment_op, AssignmentOperator, ExprParenthesized, Spanned, UnaryOperator};
use fajt_ast::{unary_op, ExprTaggedTemplate};
use fajt_ast::{update_op, UpdateOperator};
use fajt_ast::{
    Argument, Callee, Expr, ExprAssignment, ExprAwait, ExprCall, ExprConditional, ExprLiteral,
    ExprMetaProperty, ExprNew, ExprSequence, ExprThis, ExprUnary, ExprUpdate, ExprYield, Ident,
    Literal, MemberObject, Span, Super,
};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::{Token, TokenValue};
use fajt_lexer::token_matches;
use fajt_lexer::{keyword, LexerState};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses the `Expression` production.
    pub(super) fn parse_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let expr = self.parse_assignment_expr()?;

        if self.current_matches(&punct!(",")) {
            return self.parse_expr_sequence(span_start, expr);
        }

        Ok(expr)
    }

    /// Continues parsing of `Expression` if the current expression is a sequence.
    fn parse_expr_sequence(&mut self, span_start: usize, initial_expr: Expr) -> Result<Expr> {
        let mut expr = Vec::with_capacity(5);
        expr.push(initial_expr);

        loop {
            if self.maybe_consume(&punct!(","))? {
                expr.push(self.parse_assignment_expr()?);
            } else {
                break;
            }
        }

        let span = self.span_from(span_start);
        Ok(ExprSequence { span, expr }.into())
    }

    /// Parses the `AssignmentExpression` production.
    pub(super) fn parse_assignment_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        match self.current() {
            token_matches!(ok: keyword!("yield")) if self.context.is_yield => {
                self.parse_yield_expr()
            }
            token_matches!(ok: keyword!("async")) => self.parse_assignment_expr_async(),
            _ if self.is_identifier() && self.peek_matches(&punct!("=>")) => {
                self.parse_arrow_function_expr()
            }
            _ => {
                let expr = self.parse_conditional_expr()?;

                let assignment_operator = self.parse_optional_assignment_operator();
                if let Some(operator) = assignment_operator {
                    self.parse_assignment(span_start, expr, operator)
                } else {
                    Ok(expr)
                }
            }
        }
    }

    /// Parses the part of `AssignmentExpression` that starts with the `async` keyword.
    /// Examples:
    /// 1. `async => {}` // Non async arrow function with the name async.
    /// 2. `async a => {}` // Async arrow function without parameter parentheses.
    /// 3. `async(a) => {}` // Async arrow function with parentheses.
    /// 4. `async(a)` // Function call where `async` is an identifier and not a keyword.
    fn parse_assignment_expr_async(&mut self) -> Result<Expr> {
        match self.peek() {
            token_matches!(opt: punct!("=>")) => self.parse_arrow_function_expr(),
            token_matches!(opt: punct!("(")) => self.parse_cover_call_or_async_arrow_head(),
            _ if self.peek_is_identifier() => self.parse_async_arrow_function_expr(),
            _ => self.parse_conditional_expr(),
        }
    }

    /// Parses the part of `AssignmentExpression` that is an assignment.
    fn parse_assignment(
        &mut self,
        span_start: usize,
        left: Expr,
        operator: AssignmentOperator,
    ) -> Result<Expr> {
        self.validate_left_side_expr(&left, &operator)?;

        let right = self.parse_assignment_expr()?;
        let span = self.span_from(span_start);
        Ok(ExprAssignment {
            span,
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
        .into())
    }

    pub fn parse_optional_assignment_operator(&mut self) -> Option<AssignmentOperator> {
        let operator = match self.current() {
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

        if operator.is_some() {
            self.consume().unwrap();
        }

        operator
    }

    /// Parses the `ParenthesizedExpression` production.
    pub(super) fn parse_parenthesized_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();

        self.consume_assert(&punct!("("))?;
        let expr = self.parse_expr()?;
        self.consume_assert(&punct!(")"))?;

        let span = self.span_from(span_start);
        Ok(ExprParenthesized {
            span,
            expression: Box::new(expr),
        }
        .into())
    }

    /// Parses the `YieldExpression` production.
    fn parse_yield_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&keyword!("yield"))?;

        if self.is_end() || !self.stmt_not_ended() {
            let span = self.span_from(span_start);
            return Ok(ExprYield {
                span,
                argument: None,
                delegate: false,
            }
            .into());
        }

        let delegate = self.maybe_consume(&punct!("*"))?;
        let argument = self.parse_assignment_expr()?;
        let span = self.span_from(span_start);
        Ok(ExprYield {
            span,
            argument: Some(Box::new(argument)),
            delegate,
        }
        .into())
    }

    /// Parses the `ConditionalExpression` production.
    fn parse_conditional_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let expr = self.parse_short_circuit_expr()?;

        if self.current_matches(&punct!("?")) {
            self.consume()?;
            let consequent = self.parse_assignment_expr()?;

            self.consume_assert(&punct!(":"))?;

            let alternate = self.parse_assignment_expr()?;
            let span = self.span_from(span_start);
            Ok(ExprConditional {
                span,
                condition: Box::new(expr),
                consequent: Box::new(consequent),
                alternate: Box::new(alternate),
            }
            .into())
        } else {
            Ok(expr)
        }
    }

    /// Parses the `UnaryExpression` production.
    pub(super) fn parse_unary_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();

        let unary_operator = self.parse_optional_unary_operator();
        if let Some(operator) = unary_operator {
            return self.parse_unary_expr_with_operator(span_start, operator);
        }

        if self.context.is_await && token_matches!(self.current()?, keyword!("await")) {
            return self.parse_await_expr();
        }

        self.parse_update_expr()
    }

    /// Parses a unary expression when operator is already consumed.
    fn parse_unary_expr_with_operator(
        &mut self,
        span_start: usize,
        operator: UnaryOperator,
    ) -> Result<Expr> {
        let argument = self.parse_unary_expr()?;

        if operator == unary_op!("delete") {
            self.validate_delete_argument(&argument)?;
        }

        let span = self.span_from(span_start);
        Ok(ExprUnary {
            span,
            operator,
            argument: Box::new(argument),
        }
        .into())
    }

    fn parse_optional_unary_operator(&mut self) -> Option<UnaryOperator> {
        let operator = match self.current() {
            token_matches!(ok: punct!("+")) => Some(unary_op!("+")),
            token_matches!(ok: punct!("-")) => Some(unary_op!("-")),
            token_matches!(ok: punct!("~")) => Some(unary_op!("~")),
            token_matches!(ok: punct!("!")) => Some(unary_op!("!")),
            token_matches!(ok: keyword!("delete")) => Some(unary_op!("delete")),
            token_matches!(ok: keyword!("void")) => Some(unary_op!("void")),
            token_matches!(ok: keyword!("typeof")) => Some(unary_op!("typeof")),
            _ => None,
        };

        if operator.is_some() {
            self.consume().unwrap();
        }

        operator
    }

    /// Parses the `AwaitExpression` production.
    fn parse_await_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&keyword!("await"))?;

        let argument = self.parse_unary_expr()?.into();

        let span = self.span_from(span_start);
        Ok(ExprAwait { span, argument }.into())
    }

    /// Parses the `UpdateExpression` production.
    fn parse_update_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();

        let prefix_operator = self.parse_optional_update_operator();
        if let Some(operator) = prefix_operator {
            return self.parse_prefix_update_expr(span_start, operator);
        }

        let expr = self.parse_left_hand_side_expr()?;

        // New line is not allowed between argument and postfix update operator.
        // If current token is first on line, we are not parsing postfix update expression.
        if self.first_on_line() {
            return Ok(expr);
        }

        let postfix_operator = self.parse_optional_update_operator();
        if let Some(operator) = postfix_operator {
            self.parse_postfix_update_expr(expr, operator)
        } else {
            Ok(expr)
        }
    }

    /// Parses the `++/-- UnaryExpression` of the `UpdateExpression` production.
    fn parse_prefix_update_expr(
        &mut self,
        span_start: usize,
        operator: UpdateOperator,
    ) -> Result<Expr> {
        let argument = self.parse_unary_expr()?;
        self.validate_update_expression_argument(&argument)?;

        let span = self.span_from(span_start);
        Ok(ExprUpdate {
            span,
            operator,
            prefix: true,
            argument: Box::new(argument),
        }
        .into())
    }

    /// Parses the `UnaryExpression ++/--` of the `UpdateExpression` production.
    fn parse_postfix_update_expr(
        &mut self,
        argument: Expr,
        operator: UpdateOperator,
    ) -> Result<Expr> {
        self.validate_update_expression_argument(&argument)?;

        let span = self.span_from(argument.span().start);
        Ok(ExprUpdate {
            span,
            operator,
            prefix: false,
            argument: Box::new(argument),
        }
        .into())
    }

    /// Parses the prefix or postfix operator of an update expression.
    fn parse_optional_update_operator(&mut self) -> Option<UpdateOperator> {
        let operator = match self.current() {
            token_matches!(ok: punct!("++")) => Some(update_op!("++")),
            token_matches!(ok: punct!("--")) => Some(update_op!("--")),
            _ => None,
        };

        if operator.is_some() {
            self.consume().unwrap();
        }

        operator
    }

    /// Parses the `LeftHandSideExpression` production and non recursive parts of the
    /// `CallExpression` production.
    pub(super) fn parse_left_hand_side_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let expr = match self.current() {
            token_matches!(ok: keyword!("new")) => self.parse_new_or_member_expr(),
            token_matches!(ok: keyword!("super")) if self.peek_matches(&punct!("(")) => {
                self.parse_super_call_expr()
            }
            token_matches!(ok: keyword!("import")) if self.peek_matches(&punct!("(")) => {
                self.parse_import_call_expr()
            }
            _ => {
                let span_start = self.position();
                let expr = self.parse_new_or_member_expr()?;
                let call_expr = self.parse_recursive_call_expression(span_start, expr)?;
                Ok(call_expr)
            }
        }?;

        if self.current_matches(&punct!("?.")) {
            // The NewExpression production handles nested news and is not included in the
            // OptionalExpression production as a base for the chain.
            if expr.is_nested_new() {
                return Err(Error::unexpected_token(self.consume()?));
            }

            self.parse_optional_expr(span_start, expr)
        } else {
            Ok(expr)
        }
    }

    /// Parses the recursive parts of the `CallExpression` production.
    fn parse_recursive_call_expression(
        &mut self,
        span_start: usize,
        previous_expr: Expr,
    ) -> Result<Expr> {
        let mut expr = previous_expr;
        loop {
            expr = match self.current() {
                token_matches!(ok: punct!("(")) => self.parse_call_expr(span_start, expr)?,
                token_matches!(ok: punct!(".") | punct!("[")) => {
                    let left = MemberObject::Expr(Box::new(expr));
                    self.parse_member_expr_right_side(span_start, left)?
                }
                token_matches!(ok: TokenValue::Literal(Literal::Template(_))) => {
                    self.parse_tagged_template(span_start, expr)?
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    /// Parses the `CallExpression Arguments` of the `CallExpression` production.
    pub(super) fn parse_call_expr(&mut self, span_start: usize, callee: Expr) -> Result<Expr> {
        let (arguments_span, arguments) = self.parse_arguments()?;
        let span = self.span_from(span_start);
        Ok(ExprCall {
            span,
            callee: Callee::Expr(Box::new(callee)),
            arguments_span,
            arguments,
        }
        .into())
    }

    /// Parses the `SuperCall` production.
    fn parse_super_call_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&keyword!("super"))?;

        let (arguments_span, arguments) = self.parse_arguments()?;
        let span = self.span_from(span_start);

        if !self.context.super_call_allowed {
            return Err(Error::syntax_error(
                "super() now allowed here".to_owned(),
                span,
            ));
        }

        Ok(ExprCall {
            span,
            callee: Callee::Super,
            arguments_span,
            arguments,
        }
        .into())
    }

    /// Parses the `ImportCall` production.
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

        self.consume_assert(&punct!("("))?;
        let expr = self.parse_assignment_expr()?;
        self.consume_assert(&punct!(")"))?;

        let span = self.span_from(span_start);
        Ok((span, vec![Argument::Expr(expr)]))
    }

    /// Parses the `MemberExpression` and `NewExpression` productions.
    pub fn parse_new_or_member_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let mut expr = self.parse_member_expr_from_terminal()?;

        loop {
            expr = match self.current() {
                token_matches!(ok: punct!(".") | punct!("[")) => {
                    let left = MemberObject::Expr(Box::new(expr));
                    self.parse_member_expr_right_side(span_start, left)?
                }
                Ok(token_matches!(@template)) => self.parse_tagged_template(span_start, expr)?,
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parses a tagged template.
    fn parse_tagged_template(&mut self, span_start: usize, callee: Expr) -> Result<Expr> {
        let template = self.parse_template_literal()?;
        let span = self.span_from(span_start);
        Ok(ExprTaggedTemplate {
            span,
            callee: Box::new(callee),
            template,
        }
        .into())
    }

    /// Parses the parts of `MemberExpressions` that can be decided from terminals.
    fn parse_member_expr_from_terminal(&mut self) -> Result<Expr> {
        match self.current()? {
            token_matches!(keyword!("super")) => self.parse_super_property(),
            token_matches!(keyword!("new")) => {
                if self.peek_matches(&punct!(".")) {
                    self.parse_new_target()
                } else {
                    self.parse_new_expr()
                }
            }
            token_matches!(keyword!("import")) if self.peek_matches(&punct!(".")) => {
                self.parse_import_meta()
            }
            _ => self.parse_primary_expr(),
        }
    }

    /// Parses the `new NewExpression` production.
    fn parse_new_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume()?;

        let callee = self.parse_new_or_member_expr()?;
        let (arguments_span, arguments) = self.parse_optional_arguments()?;

        let span = self.span_from(span_start);
        Ok(ExprNew {
            span,
            callee: Box::new(callee),
            arguments_span,
            arguments,
        }
        .into())
    }

    /// Parses the `SuperProperty` production.
    fn parse_super_property(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let super_token = self.consume()?;

        if !token_matches!(self.current(), ok: punct!(".") | punct!("[")) {
            let span = self.span_from(span_start);
            return Err(Error::syntax_error(
                "`super` keyword not expected here".to_string(),
                span,
            ));
        }

        if !self.context.in_method {
            let span = self.span_from(span_start);
            return Err(Error::syntax_error(
                "`super` property access only valid inside methods".to_string(),
                span,
            ));
        }

        self.parse_member_expr_right_side(
            span_start,
            MemberObject::Super(Super {
                span: super_token.span,
            }),
        )
    }

    /// Parses the `ImportMeta` production.
    fn parse_import_meta(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let import_token = self.consume()?;
        self.consume()?; // .

        let property = self.parse_identifier()?;
        if property.name != "meta" {
            return Err(Error::unexpected_identifier(property));
        }

        let span = self.span_from(span_start);
        Ok(ExprMetaProperty {
            span,
            meta: Ident::new("import", import_token.span),
            property,
        }
        .into())
    }

    /// Parses the `NewTarget` production.
    fn parse_new_target(&mut self) -> Result<Expr> {
        let span_start = self.position();
        let new_token = self.consume()?;
        self.consume()?; // .

        let property = self.parse_identifier()?;
        if property.name != "target" {
            return Err(Error::unexpected_identifier(property));
        }

        let span = self.span_from(span_start);
        Ok(ExprMetaProperty {
            span,
            meta: Ident::new("new", new_token.span),
            property,
        }
        .into())
    }

    /// Parses the `Arguments` production.
    pub(super) fn parse_arguments(&mut self) -> Result<(Span, Vec<Argument>)> {
        let span_start = self.position();
        self.consume_assert(&punct!("("))?;

        let mut arguments = Vec::new();
        loop {
            if self.maybe_consume(&punct!(")"))? {
                break;
            }

            arguments.push(self.parse_argument()?);
            self.consume_list_delimiter(&punct!(")"))?;
        }

        let span = self.span_from(span_start);
        Ok((span, arguments))
    }

    fn parse_argument(&mut self) -> Result<Argument> {
        Ok(if self.maybe_consume(&punct!("..."))? {
            Argument::Spread(self.parse_assignment_expr()?)
        } else {
            Argument::Expr(self.parse_assignment_expr()?)
        })
    }

    /// Parses arguments where optional.
    pub(super) fn parse_optional_arguments(&mut self) -> Result<(Option<Span>, Vec<Argument>)> {
        if self.current_matches(&punct!("(")) {
            self.parse_arguments()
                .map(|(span, args)| (Some(span), args))
        } else {
            Ok((None, Vec::new()))
        }
    }

    /// Parses the `PrimaryExpression` production.
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
                .with_context(self.context.with_strict(true))
                .parse_class_expr()?,
            token_matches!(keyword!("async")) if !self.followed_by_new_line() => {
                self.parse_async_function_expr()?
            }
            token_matches!(punct!("/")) => {
                // The lexer do not account for regexp by default because it is ambiguous with other
                // productions. But if we find a '/' punctuator here it must be a misinterpreted
                // regexp literal, so we re-read with current token with correct state and the lexer
                // will produce the regexp literal for us to consume.
                self.reader.reread_with_state(LexerState::regex_allowed())?;
                self.parse_regexp_literal()?
            }
            token_matches!(@template-head) => self.parse_template_literal_expr()?,
            token_matches!(punct!("(")) => self.parse_cover_parenthesized_and_arrow_parameters()?,
            _ => {
                // To avoid errors like "unexpected token `.`, expected identifier" as default
                // errors, only parse the identifier if it may be a identifier in any context
                // otherwise fall back to generic unexpected token.
                if self.with_context(Context::default()).is_identifier() {
                    self.parse_identifier_reference()?
                } else {
                    return Err(Error::unexpected_token(self.consume()?));
                }
            }
        })
    }

    /// Parses the `RegularExpressionLiteral` production.
    fn parse_regexp_literal(&mut self) -> Result<Expr> {
        let regexp = self.parse_literal()?;
        debug_assert!(matches!(
            regexp,
            Expr::Literal(ExprLiteral {
                literal: Literal::Regexp(_),
                ..
            })
        ));

        Ok(regexp)
    }

    /// Parses the `this` expression which is part of the `PrimaryExpression` production.
    fn parse_this_expr(&mut self) -> Result<Expr> {
        let token = self.consume_assert(&keyword!("this"))?;
        Ok(ExprThis::new(token.span).into())
    }

    /// Parses the `IdentifierReference` production.
    fn parse_identifier_reference(&mut self) -> Result<Expr> {
        let ident = self.parse_identifier()?;
        Ok(ident.into())
    }

    /// Parses the `Initializer` production.
    pub(super) fn parse_initializer(&mut self) -> Result<Expr> {
        self.consume()?; // Skip =
        self.parse_assignment_expr()
    }
}
