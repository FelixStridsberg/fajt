use crate::ast::{
    CatchClause, Stmt, StmtBlock, StmtBreak, StmtContinue, StmtDebugger, StmtEmpty, StmtExpr,
    StmtIf, StmtReturn, StmtSwitch, StmtThrow, StmtTry, StmtWith, SwitchCase, VariableKind,
};
use crate::error::{Result, ThenTry};
use crate::{ContextModify, Parser};
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    pub(super) fn parse_stmt(&mut self) -> Result<Stmt> {
        Ok(match self.reader.current()? {
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
            // TODO LabelledStatement
            // TODO IterationStatement
            token_matches!(keyword!("do")) => self.parse_do_while_stmt()?,
            token_matches!(keyword!("while")) => self.parse_while_stmt()?,
            token_matches!(keyword!("for")) => self.parse_for_stmt()?,
            token_matches!(keyword!("switch")) => self.parse_switch_stmt()?,
            _ if self.is_expr_stmt()? => self
                .with_context(ContextModify::new().set_in(true))
                .parse_expr_stmt()?,

            // Declarations are handles as statements
            token_matches!(keyword!("function")) => self.parse_function_declaration()?,
            token_matches!(keyword!("async")) if self.peek_matches(keyword!("function")) => {
                self.parse_async_function_declaration()?
            }
            t => unimplemented!("Invalid statement error handling {:?}", t),
        })
    }

    /// Check if current position matches the start of an expression statement as specified in the
    /// `ExpressionStatement` goal symbol.
    fn is_expr_stmt(&self) -> Result<bool> {
        if matches!(
            self.reader.current()?.value,
            punct!("{") | keyword!("function") | keyword!("class")
        ) {
            return Ok(false);
        }

        if self.current_matches(keyword!("let")) && self.peek_matches(punct!("[")) {
            return Ok(false);
        }

        if self.current_matches(keyword!("async")) && self.peek_matches(keyword!("function")) {
            return Ok(self.followed_by_new_lined());
        }

        Ok(true)
    }

    /// Parses the `BlockStatement` goal symbol.
    fn parse_block_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(punct!("{"))?;

        let mut statements = Vec::new();
        loop {
            if self.current_matches(punct!("}")) {
                self.reader.consume()?;
                break;
            } else {
                let statement = self.parse_stmt()?;
                statements.push(statement)
            }
        }

        let span = self.span_from(span_start);
        Ok(StmtBlock { span, statements }.into())
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        let expr = self.parse_expr()?;

        self.maybe_consume_semicolon()?;

        let span = self.span_from(span_start);
        Ok(StmtExpr { span, expr }.into())
    }

    /// Parses the `EmptyStatement` goal symbol.
    fn parse_empty_stmt(&mut self) -> Result<Stmt> {
        let token = self.consume_assert(punct!(";"))?;
        Ok(StmtEmpty { span: token.span }.into())
    }

    /// Parses the `BreakStatement` goal symbol.
    fn parse_break_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("break"))?;

        let label = self.stmt_not_ended().then_try(|| self.parse_identifier())?;
        self.maybe_consume_semicolon()?;

        let span = self.span_from(span_start);
        Ok(StmtBreak { span, label }.into())
    }

    /// Parses the `ContinueStatement` goal symbol.
    fn parse_continue_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("continue"))?;

        let label = self.stmt_not_ended().then_try(|| self.parse_identifier())?;
        self.maybe_consume_semicolon()?;

        let span = self.span_from(span_start);
        Ok(StmtContinue { span, label }.into())
    }

    /// Parses the `ReturnStatement` goal symbol.
    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("return"))?;

        let argument = self.stmt_not_ended().then_try(|| self.parse_expr())?;
        self.maybe_consume_semicolon()?;

        let span = self.span_from(span_start);
        Ok(StmtReturn { span, argument }.into())
    }

    /// Parses the `ThrowStatement` goal symbol.
    fn parse_throw_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("throw"))?;

        let argument = self.stmt_not_ended().then_try(|| self.parse_expr())?;
        let span = self.span_from(span_start);
        Ok(StmtThrow { span, argument }.into())
    }

    /// True if the current token is not preceded by a line feed or is a semi colon.
    fn stmt_not_ended(&self) -> bool {
        match self.reader.current() {
            token_matches!(ok: punct!(";")) | Err(_) => false,
            Ok(token) if token.first_on_line => false,
            _ => true,
        }
    }

    /// Parses the `DebuggerStatement` goal symbol.
    fn parse_debugger_stmt(&mut self) -> Result<Stmt> {
        let token = self.consume_assert(keyword!("debugger"))?;
        Ok(StmtDebugger { span: token.span }.into())
    }

    /// Parses the `IfStatement` goal symbol.
    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("if"))?;
        self.consume_assert(punct!("("))?;
        let condition = self.parse_expr()?;
        self.consume_assert(punct!(")"))?;

        let consequent = self.parse_stmt()?;
        let alternate = self.current_matches(keyword!("else")).then_try(|| {
            self.reader.consume()?;
            Ok(Box::new(self.parse_stmt()?))
        })?;

        let span = self.span_from(span_start);
        Ok(StmtIf {
            span,
            condition,
            consequent: consequent.into(),
            alternate,
        }
        .into())
    }

    /// Parses the `WithStatement` goal symbol.
    fn parse_with_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("with"))?;
        self.consume_assert(punct!("("))?;
        let object = self.parse_expr()?;
        self.consume_assert(punct!(")"))?;

        let body = self.parse_stmt()?;
        let span = self.span_from(span_start);
        Ok(StmtWith {
            span,
            object,
            body: body.into(),
        }
        .into())
    }

    /// Parses the `TryStatement` goal symbol.
    fn parse_try_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("try"))?;

        let block = self.parse_block_stmt()?.unwrap_block_stmt();
        let handler = self
            .current_matches(keyword!("catch"))
            .then_try(|| self.parse_catch_clause())?;
        let finalizer = self.current_matches(keyword!("finally")).then_try(|| {
            self.reader.consume()?;
            Ok(self.parse_block_stmt()?.unwrap_block_stmt())
        })?;

        let span = self.span_from(span_start);
        Ok(StmtTry {
            span,
            block,
            handler,
            finalizer,
        }
        .into())
    }

    fn parse_catch_clause(&mut self) -> Result<CatchClause> {
        let span_start = self.position();
        self.consume_assert(keyword!("catch"))?;

        let parameter = self.current_matches(punct!("(")).then_try(|| {
            self.reader.consume()?;
            let pattern = self.parse_binding_pattern()?;
            self.consume_assert(punct!(")"))?;
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

    fn parse_switch_stmt(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("switch"))?;
        self.consume_assert(punct!("("))?;

        let discriminant = self.parse_expr()?;
        self.consume_assert(punct!(")"))?;

        let cases = self.parse_switch_cases()?;

        let span = self.span_from(span_start);
        Ok(StmtSwitch {
            span,
            discriminant,
            cases,
        }
        .into())
    }

    fn parse_switch_cases(&mut self) -> Result<Vec<SwitchCase>> {
        self.consume_assert(punct!("{"))?;

        let mut cases = Vec::new();
        loop {
            if self.current_matches(punct!("}")) {
                break;
            }

            cases.push(self.parse_switch_case()?);
        }

        self.consume_assert(punct!("}"))?;

        Ok(cases)
    }

    fn parse_switch_case(&mut self) -> Result<SwitchCase> {
        let span_start = self.position();
        let test = if self.current_matches(keyword!("case")) {
            self.consume_assert(keyword!("case"))?;
            let test = self.parse_expr()?;
            self.consume_assert(punct!(":"))?;
            Some(test)
        } else {
            self.consume_assert(keyword!("default"))?;
            self.consume_assert(punct!(":"))?;
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
                self.reader.current(),
                ok: punct!("}") | keyword!("case") | keyword!("default")
            ) {
                break;
            }

            statements.push(self.parse_stmt()?);
        }

        Ok(statements)
    }
}
