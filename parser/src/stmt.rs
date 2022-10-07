use crate::error::Result;
use crate::{Error, Parser, ThenTry};
use fajt_ast::{
    CatchClause, SourceType, Stmt, StmtBlock, StmtBreak, StmtContinue, StmtDebugger, StmtEmpty,
    StmtExpr, StmtIf, StmtLabeled, StmtReturn, StmtSwitch, StmtThrow, StmtTry, StmtWith,
    SwitchCase, VariableKind,
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
    pub(super) fn parse_all_stmts(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        loop {
            if self.is_end() {
                break;
            }

            stmts.push(self.parse_stmt()?);
        }

        Ok(stmts)
    }

    // Parses statements, we handle declarations as statements as well since they appear in the same
    // contexts.
    pub(super) fn parse_stmt(&mut self) -> Result<Stmt> {
        Ok(match self.current()? {
            token_matches!(punct!(";")) => self.parse_empty_stmt()?,
            token_matches!(punct!("{")) => self.parse_block_stmt()?,
            token_matches!(keyword!("var")) => self.parse_variable_stmt(VariableKind::Var)?,
            token_matches!(keyword!("const")) => self.parse_variable_stmt(VariableKind::Const)?,
            token_matches!(keyword!("let")) => self.parse_variable_stmt(VariableKind::Let)?,
            token_matches!(keyword!("if")) => self.parse_if_stmt()?,
            token_matches!(keyword!("break")) => self.parse_break_stmt()?,
            token_matches!(keyword!("continue")) => self.parse_continue_stmt()?,
            token_matches!(keyword!("return")) => self.parse_return_stmt()?,
            token_matches!(keyword!("with")) => self.parse_with_stmt()?,
            token_matches!(keyword!("throw")) => self.parse_throw_stmt()?,
            token_matches!(keyword!("try")) => self.parse_try_stmt()?,
            token_matches!(keyword!("debugger")) => self.parse_debugger_stmt()?,
            token_matches!(keyword!("do")) => self.parse_do_while_stmt()?,
            token_matches!(keyword!("while")) => self.parse_while_stmt()?,
            token_matches!(keyword!("for")) => self.parse_for_stmt()?,
            token_matches!(keyword!("switch")) => self.parse_switch_stmt()?,
            token_matches!(keyword!("function")) => self.parse_function_declaration()?,
            token_matches!(keyword!("async")) if self.peek_matches(&keyword!("function")) => {
                self.parse_async_function_declaration()?
            }
            token_matches!(keyword!("class")) => self
                .with_context(self.context.with_strict(true))
                .parse_class_decl()?,
            token_matches!(keyword!("import")) => {
                if self.source_type() == SourceType::Script {
                    return Err(Error::syntax_error(
                        "Cannot use `import` statement outside a module".to_owned(),
                        self.current()?.span.clone(),
                    ));
                }

                self.set_source_type(SourceType::Module);
                self.parse_import_declaration()?
            }
            token_matches!(keyword!("export")) => {
                if self.source_type() == SourceType::Script {
                    return Err(Error::syntax_error(
                        "Cannot use `export` statement outside a module".to_owned(),
                        self.current()?.span.clone(),
                    ));
                }

                self.set_source_type(SourceType::Module);
                self.parse_export_declaration()?
            }
            _ if self.is_identifier() && self.peek_matches(&punct!(":")) => {
                self.parse_labeled_stmt()?
            }
            _ if self.is_expr_stmt()? => self
                .with_context(self.context.with_in(true))
                .parse_expr_stmt()?,
            t => unimplemented!("Invalid statement error handling {:?}", t),
        })
    }

    /// Check if current position matches the start of an expression statement as specified in the
    /// `ExpressionStatement` production.
    fn is_expr_stmt(&self) -> Result<bool> {
        if matches!(
            self.current()?.value,
            punct!("{") | keyword!("function") | keyword!("class")
        ) {
            return Ok(false);
        }

        if self.current_matches(&keyword!("let")) && self.peek_matches(&punct!("[")) {
            return Ok(false);
        }

        if self.current_matches(&keyword!("async")) && self.peek_matches(&keyword!("function")) {
            return Ok(self.followed_by_new_line());
        }

        Ok(true)
    }

    /// Parses the `BlockStatement` production.
    fn parse_block_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&punct!("{"))?;

        let mut statements = Vec::new();
        loop {
            if self.maybe_consume(&punct!("}"))? {
                break;
            } else {
                let statement = self.parse_stmt()?;
                statements.push(statement)
            }
        }

        let span = self.span_from(span_start);
        Ok(StmtBlock { span, statements }.into())
    }

    /// Parses the `ExpressionStatement` production.
    fn parse_expr_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        let expr = self.with_context(self.context.with_in(true)).parse_expr()?;

        self.consume_optional_semicolon()?;

        let span = self.span_from(span_start);
        Ok(StmtExpr {
            span,
            expr: Box::new(expr),
        }
        .into())
    }

    /// Parses the `LabeledStatement` production.
    fn parse_labeled_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        let label = self.parse_identifier()?;
        self.consume_assert(&punct!(":"))?;
        let body = self.parse_stmt()?;
        let span = self.span_from(span_start);
        Ok(StmtLabeled {
            span,
            label,
            body: Box::new(body),
        }
        .into())
    }

    /// Parses the `EmptyStatement` production.
    fn parse_empty_stmt(&mut self) -> Result<Stmt> {
        let token = self.consume_assert(&punct!(";"))?;
        Ok(StmtEmpty { span: token.span }.into())
    }

    /// Parses the `BreakStatement` production.
    fn parse_break_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("break"))?;

        let label = (!self.stmt_ended()).then_try(|| self.parse_identifier())?;
        self.maybe_consume(&punct!(";"))?;

        let span = self.span_from(span_start);
        Ok(StmtBreak { span, label }.into())
    }

    /// Parses the `ContinueStatement` production.
    fn parse_continue_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("continue"))?;

        let label = (!self.stmt_ended()).then_try(|| self.parse_identifier())?;
        self.maybe_consume(&punct!(";"))?;

        let span = self.span_from(span_start);
        Ok(StmtContinue { span, label }.into())
    }

    /// Parses the `ReturnStatement` production.
    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("return"))?;

        let argument = (!self.stmt_ended())
            .then_try(|| self.with_context(self.context.with_in(true)).parse_expr())?;
        self.maybe_consume(&punct!(";"))?;

        let span = self.span_from(span_start);
        Ok(StmtReturn {
            span,
            argument: argument.map(Box::new),
        }
        .into())
    }

    /// Parses the `ThrowStatement` production.
    fn parse_throw_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        let throw = self.consume_assert(&keyword!("throw"))?;

        match self.current() {
            token_matches!(ok: punct!(";")) | Err(_) => {
                return Err(Error::unexpected_token(self.consume()?));
            }
            Ok(token) if token.first_on_line => {
                return Err(Error::syntax_error(
                    "Illegal newline after throw".to_owned(),
                    throw.span,
                ));
            }
            _ => {}
        }

        let argument = self.parse_expr()?;
        self.maybe_consume(&punct!(";"))?;

        let span = self.span_from(span_start);
        Ok(StmtThrow {
            span,
            argument: Box::new(argument),
        }
        .into())
    }

    /// Parses the `DebuggerStatement` production.
    fn parse_debugger_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();

        self.consume_assert(&keyword!("debugger"))?;
        self.maybe_consume(&punct!(";"))?;

        let span = self.span_from(span_start);
        Ok(StmtDebugger { span }.into())
    }

    /// Parses the `IfStatement` production.
    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();

        self.consume_assert(&keyword!("if"))?;
        self.consume_assert(&punct!("("))?;
        let condition = self.with_context(self.context.with_in(true)).parse_expr()?;
        self.consume_assert(&punct!(")"))?;

        let consequent = self.parse_stmt()?;
        let alternate = self
            .maybe_consume(&keyword!("else"))?
            .then_try(|| self.parse_stmt())?;

        let span = self.span_from(span_start);
        Ok(StmtIf {
            span,
            condition: Box::new(condition),
            consequent: Box::new(consequent),
            alternate: alternate.map(Box::new),
        }
        .into())
    }

    /// Parses the `WithStatement` production.
    fn parse_with_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();

        self.consume_assert(&keyword!("with"))?;
        self.consume_assert(&punct!("("))?;
        let object = self.parse_expr()?;
        self.consume_assert(&punct!(")"))?;

        let body = self.parse_stmt()?;

        let span = self.span_from(span_start);
        Ok(StmtWith {
            span,
            object: Box::new(object),
            body: Box::new(body),
        }
        .into())
    }

    /// Parses the `TryStatement` production.
    fn parse_try_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();

        self.consume_assert(&keyword!("try"))?;
        let block = self.parse_block_stmt()?.unwrap_block_stmt();

        let handler = self
            .current_matches(&keyword!("catch"))
            .then_try(|| self.parse_catch_clause())?;

        let finalizer = self
            .maybe_consume(&keyword!("finally"))?
            .then_try(|| Ok(self.parse_block_stmt()?.unwrap_block_stmt()))?;

        let span = self.span_from(span_start);
        Ok(StmtTry {
            span,
            block,
            handler: handler.map(Box::new),
            finalizer,
        }
        .into())
    }

    /// Parses the `Catch` production.
    fn parse_catch_clause(&mut self) -> Result<CatchClause> {
        let span_start = self.position();

        self.consume_assert(&keyword!("catch"))?;
        let parameter = self.maybe_consume(&punct!("("))?.then_try(|| {
            let pattern = self.parse_binding_pattern()?;
            self.consume_assert(&punct!(")"))?;
            Ok(pattern)
        })?;

        let body = self.parse_block_stmt()?.unwrap_block_stmt();

        let span = self.span_from(span_start);
        Ok(CatchClause {
            span,
            parameter,
            body,
        })
    }

    /// Parses the `SwitchStatement` production.
    fn parse_switch_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();

        self.consume_assert(&keyword!("switch"))?;
        self.consume_assert(&punct!("("))?;

        let discriminant = self.parse_expr()?;
        self.consume_assert(&punct!(")"))?;

        let cases = self.parse_case_block()?;

        let span = self.span_from(span_start);
        Ok(StmtSwitch {
            span,
            discriminant: Box::new(discriminant),
            cases,
        }
        .into())
    }

    /// Parses the `CaseBlock` production.
    fn parse_case_block(&mut self) -> Result<Vec<SwitchCase>> {
        self.consume_assert(&punct!("{"))?;

        let mut cases = Vec::new();
        loop {
            if self.current_matches(&punct!("}")) {
                break;
            }

            cases.push(self.parse_case_clause()?);
        }

        self.consume_assert(&punct!("}"))?;

        Ok(cases)
    }

    /// Parses the `CaseClause` production.
    fn parse_case_clause(&mut self) -> Result<SwitchCase> {
        let span_start = self.position();
        let test = if self.maybe_consume(&keyword!("case"))? {
            let test = self.parse_expr()?;
            self.consume_assert(&punct!(":"))?;
            Some(test)
        } else {
            self.consume_assert(&keyword!("default"))?;
            self.consume_assert(&punct!(":"))?;
            None
        };

        let consequent = self.parse_switch_case_stmt_list()?;
        let span = self.span_from(span_start);
        Ok(SwitchCase {
            span,
            test,
            consequent,
        })
    }

    fn parse_switch_case_stmt_list(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        loop {
            if token_matches!(
                self.current(),
                ok: punct!("}") | keyword!("case") | keyword!("default")
            ) {
                break;
            }

            statements.push(self.parse_stmt()?);
        }

        Ok(statements)
    }

    /// Consumes semicolon if exists, returns error if no semicolon exists and no semicolon can be
    /// auto inserted.
    pub(super) fn consume_optional_semicolon(&mut self) -> Result<()> {
        if !self.maybe_consume(&punct!(";"))? && !self.can_insert_semicolon() {
            Err(Error::unexpected_token(self.consume()?))
        } else {
            Ok(())
        }
    }
}
