use crate::error::{Error, Result};
use crate::{Parser, ThenTry};
use fajt_ast::{BindingPattern, Stmt, StmtVariable, VariableDeclaration, VariableKind};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::token::Token;
use fajt_lexer::{punct, LexerState};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses the `VariableStatement` or `LexicalDeclaration` production.
    pub(super) fn parse_variable_stmt(&mut self, kind: VariableKind) -> Result<Stmt> {
        let token = self.consume()?;
        let span_start = token.span.start;

        let declarations = if kind == VariableKind::Var {
            self.with_context(self.context.with_in(true))
                .parse_variable_declarations()?
        } else {
            self.parse_variable_declarations()?
        };
        self.maybe_consume(&punct!(";"))?;

        let span = self.span_from(span_start);
        Ok(StmtVariable {
            span,
            kind,
            declarations,
        }
        .into())
    }

    pub(super) fn parse_variable_declarations(&mut self) -> Result<Vec<VariableDeclaration>> {
        let mut declarations = vec![self.parse_variable_declaration()?];

        loop {
            if self.maybe_consume(&punct!(","))? {
                declarations.push(self.parse_variable_declaration()?);
            } else {
                break;
            }
        }

        Ok(declarations)
    }

    pub(super) fn peek_matches_lexical_binding(&self) -> bool {
        self.peek_matches(&punct!("{"))
            || self.peek_matches(&punct!("["))
            || self.peek_is_identifier()
    }

    /// Parses the `VariableDeclaration` or `LexicalBinding` production.
    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let span_start = self.position();
        let pattern = self.parse_binding_pattern()?;

        if !matches!(pattern, BindingPattern::Ident(_)) && !self.current_matches(&punct!("=")) {
            let span = self.span_from(span_start);
            return Err(Error::syntax_error(
                "Missing initializer in destructuring declaration".to_owned(),
                span,
            ));
        }

        let initializer = self
            .current_matches(&punct!("="))
            .then_try(|| self.parse_initializer())?;
        let span = self.span_from(span_start);
        Ok(VariableDeclaration {
            span,
            pattern,
            initializer,
        })
    }
}
