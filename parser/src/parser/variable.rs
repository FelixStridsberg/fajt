use crate::ast::{Statement, VariableDeclaration, VariableKind, VariableStatement};
use crate::error::{Result, ThenTry};
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::punct;
use fajt_lexer::token::{Span, Token};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    /// Parses the `VariableStatement` or `LexicalDeclaration` goal symbol.
    pub(super) fn parse_variable_statement(&mut self, kind: VariableKind) -> Result<Statement> {
        let token = self.reader.consume()?;
        let start = token.span.start;

        let declarations = self.parse_variable_declarations()?;
        let end = self.reader.position();

        let span = Span::new(start, end);
        Ok(VariableStatement {
            span,
            kind,
            declarations,
        }
        .into())
    }

    fn parse_variable_declarations(&mut self) -> Result<Vec<VariableDeclaration>> {
        let mut declarations = Vec::new();
        declarations.push(self.parse_variable_declaration()?);

        loop {
            if self.current_matches(punct!(",")) {
                self.reader.consume()?;
                declarations.push(self.parse_variable_declaration()?);
            } else {
                break;
            }
        }

        if self.current_matches(punct!(";")) {
            self.reader.consume()?;
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
