use crate::ast::{ClassExpression, Expression};
use crate::error::Result;
use crate::Parser;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token_matches;

impl Parser<'_, '_> {
    /// Parses the `ClassExpression` goal symbol.
    pub(super) fn parse_class_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("class")));

        let identifier = self
            .is_identifier()
            .then(|| self.parse_identifier().unwrap());

        let super_class = if token_matches!(self.reader.current(), ok: keyword!("extends")) {
            self.reader.consume()?;
            Some(self.parse_left_hand_side_expression()?)
        } else {
            None
        };

        if token_matches!(self.reader.current(), ok: punct!("{")) {
            self.reader.consume()?;
            if !token_matches!(self.reader.current(), ok: punct!("}")) {
                todo!();
            }

            self.reader.consume()?;
        }

        let span = self.span_from(span_start);
        Ok(ClassExpression {
            span,
            identifier,
            super_class,
            body: vec![],
        }
        .into())
    }
}
