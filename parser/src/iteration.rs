use crate::error::Result;
use crate::static_semantics::ExprSemantics;
use crate::{Error, Parser, ThenTry};
use fajt_ast::{
    AssignmentOperator, ForInit, Stmt, StmtDoWhile, StmtFor, StmtForIn, StmtForOf, StmtVariable,
    StmtWhile, VariableKind,
};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use fajt_lexer::{keyword, LexerState};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses the `DoWhileStatement` production.
    pub(super) fn parse_do_while_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("do"))?;

        let body = self.parse_stmt()?;

        self.consume_assert(&keyword!("while"))?;
        self.consume_assert(&punct!("("))?;

        let test = self.with_context(self.context.with_in(true)).parse_expr()?;

        self.consume_assert(&punct!(")"))?;
        self.maybe_consume(&punct!(";"))?;

        let span = self.span_from(span_start);
        Ok(StmtDoWhile {
            span,
            body: Box::new(body),
            test: Box::new(test),
        }
        .into())
    }

    /// Parses the `WhileStatement` production.
    pub(super) fn parse_while_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("while"))?;
        self.consume_assert(&punct!("("))?;

        let test = self.parse_expr()?;

        self.consume_assert(&punct!(")"))?;

        let body = self.parse_stmt()?;

        let span = self.span_from(span_start);
        Ok(StmtWhile {
            span,
            test: Box::new(test),
            body: Box::new(body),
        }
        .into())
    }

    /// Parses the `ForStatement` production.
    pub(super) fn parse_for_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("for"))?;

        let asynchronous = self.context.is_await && self.maybe_consume(&keyword!("await"))?;

        self.consume_assert(&punct!("("))?;

        let init = self.parse_optional_for_init()?;

        // Only for-of loops are allowed to be asynchronous.
        if asynchronous && !self.current_matches(&keyword!("of")) {
            return Err(Error::unexpected_token(self.consume()?));
        }

        if init.is_none() || self.current_matches(&punct!(";")) {
            return self.parse_for(span_start, init);
        }

        match self.current()? {
            token_matches!(keyword!("of")) => {
                self.parse_for_of(span_start, init.unwrap(), asynchronous)
            }
            token_matches!(keyword!("in")) => self.parse_for_in(span_start, init.unwrap()),
            _ => Err(Error::unexpected_token(self.consume()?)),
        }
    }

    /// Parses a standard for loop, i.e. not for in/for of.
    fn parse_for(&mut self, span_start: usize, init: Option<ForInit>) -> Result<Stmt> {
        self.consume_assert(&punct!(";"))?;

        let test = (!self.current_matches(&punct!(";")))
            .then_try(|| self.with_context(self.context.with_in(true)).parse_expr())?;
        self.consume_assert(&punct!(";"))?;

        let update = (!self.current_matches(&punct!(")")))
            .then_try(|| self.with_context(self.context.with_in(true)).parse_expr())?;
        self.consume_assert(&punct!(")"))?;

        let body = self.parse_stmt()?;
        let span = self.span_from(span_start);
        Ok(StmtFor {
            span,
            init,
            test: test.map(Box::new),
            update: update.map(Box::new),
            body: Box::new(body),
        }
        .into())
    }

    fn parse_for_in(&mut self, span_start: usize, left: ForInit) -> Result<Stmt> {
        if let ForInit::Expr(expr) = &left {
            expr.early_errors_left_hand_side_expr(&self.context, &AssignmentOperator::Assign)?;
        }

        self.consume_assert(&keyword!("in"))?;

        let right = self.with_context(self.context.with_in(true)).parse_expr()?;

        self.consume_assert(&punct!(")"))?;

        let body = self.parse_stmt()?;
        let span = self.span_from(span_start);
        Ok(StmtForIn {
            span,
            left,
            right: Box::new(right),
            body: Box::new(body),
        }
        .into())
    }

    fn parse_for_of(
        &mut self,
        span_start: usize,
        left: ForInit,
        asynchronous: bool,
    ) -> Result<Stmt> {
        if let ForInit::Expr(expr) = &left {
            expr.early_errors_left_hand_side_expr(&self.context, &AssignmentOperator::Assign)?;
        }

        self.consume_assert(&keyword!("of"))?;

        let right = self.with_context(self.context.with_in(true)).parse_expr()?;

        self.consume_assert(&punct!(")"))?;

        let body = self.parse_stmt()?;
        let span = self.span_from(span_start);
        Ok(StmtForOf {
            span,
            left,
            right: Box::new(right),
            body: Box::new(body),
            asynchronous,
        }
        .into())
    }

    pub(super) fn parse_optional_for_init(&mut self) -> Result<Option<ForInit>> {
        let span_start = self.position();

        if self.current_matches(&punct!(";")) {
            return Ok(None);
        }

        let variable_kind = self.parse_optional_variable_kind()?;
        if let Some(kind) = variable_kind {
            return Ok(Some(
                self.parse_for_init_variable_declarations(span_start, kind)?,
            ));
        }

        Ok(Some(ForInit::Expr(Box::new(
            self.with_context(self.context.with_in(false))
                .parse_expr()?,
        ))))
    }

    fn parse_for_init_variable_declarations(
        &mut self,
        span_start: usize,
        kind: VariableKind,
    ) -> Result<ForInit> {
        let declarations = self
            .with_context(self.context.with_in(false))
            .parse_variable_declarations()?;

        let span = self.span_from(span_start);
        Ok(ForInit::Declaration(StmtVariable {
            span,
            kind,
            declarations,
        }))
    }

    fn parse_optional_variable_kind(&mut self) -> Result<Option<VariableKind>> {
        let variable_kind = match self.current()? {
            token_matches!(keyword!("var")) => Some(VariableKind::Var),
            token_matches!(keyword!("let")) => Some(VariableKind::Let),
            token_matches!(keyword!("const")) => Some(VariableKind::Const),
            _ => None,
        };

        if variable_kind.is_some() {
            self.consume()?;
        }

        Ok(variable_kind)
    }
}
