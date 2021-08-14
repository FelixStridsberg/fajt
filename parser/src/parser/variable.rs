use crate::ast::{Stmt, StmtVariable, VariableDeclaration, VariableKind};
use crate::error::{Result, ThenTry};
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::punct;
use fajt_lexer::token::Token;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    /// Parses the `VariableStatement` or `LexicalDeclaration` goal symbol.
    pub(super) fn parse_variable_stmt(&mut self, kind: VariableKind) -> Result<Stmt> {
        let token = self.consume()?;
        let span_start = token.span.start;

        let declarations = self.parse_variable_declarations()?;
        self.maybe_consume(punct!(";"))?;

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
            if self.maybe_consume(punct!(","))? {
                declarations.push(self.parse_variable_declaration()?);
            } else {
                break;
            }
        }

        Ok(declarations)
    }

    /// Parses the `VariableDeclaration` or `LexicalBinding` goal symbol.
    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let span_start = self.position();
        let pattern = self.parse_binding_pattern()?;
        let initializer = self
            .current_matches(punct!("="))
            .then_try(|| self.parse_initializer())?;
        let span = self.span_from(span_start);
        Ok(VariableDeclaration {
            span,
            pattern,
            initializer,
        })
    }
}
