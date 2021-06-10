use crate::ast::{Statement, VariableDeclaration, VariableKind, VariableStatement};
use crate::error::Result;
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::punct;
use fajt_lexer::token::{Span, Token};
use fajt_lexer::token_matches;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    /// Parses the `VariableStatement` or `LexicalDeclaration` goal symbol.
    pub(super) fn parse_variable_statement(&mut self, kind: VariableKind) -> Result<Statement> {
        let token = self.reader.consume()?;
        let start = token.span.start;

        // TODO parse all declarations
        let declarations = vec![self.parse_variable_declaration()?];
        let end = self.reader.position();

        let span = Span::new(start, end);
        Ok(VariableStatement {
            span,
            kind,
            declarations,
        }
        .into())
    }

    /// Parses the `VariableDeclaration` or `LexicalBinding` goal symbol.
    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let span_start = self.position();
        let pattern = self.parse_binding_pattern()?;
        let initializer = match self.reader.current()? {
            token_matches!(punct!("=")) => Some(self.parse_initializer()?),
            token_matches!(punct!(";")) => {
                self.reader.consume()?;
                None
            }
            _ => None,
        };

        let span = self.span_from(span_start);
        Ok(VariableDeclaration {
            span,
            pattern,
            initializer,
        })
    }
}
