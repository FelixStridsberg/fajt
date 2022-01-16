use crate::error::Result;
use crate::Parser;
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use fajt_lexer::LexerState;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Assumes next token is start parenthesis, skips past matching closing parenthesis, reads
    /// token, then rewinds so next token is start parenthesis again.
    ///
    /// We do this to figure out the "cover" goals that requires reading past parenthesis.
    pub(super) fn token_after_parenthesis(&mut self) -> Result<Option<Token>> {
        let start = self.consume_assert(punct!("("))?;
        self.skip_until_closing_parenthesis()?;

        let token = self.consume().ok();

        self.reader.reread_from(&start)?;
        Ok(token)
    }

    fn skip_until_closing_parenthesis(&mut self) -> Result<()> {
        let mut depth = 1;
        loop {
            let token = self.consume()?;
            match &token {
                token_matches!(punct!("(")) => depth += 1,
                token_matches!(punct!(")")) => depth -= 1,
                _ => {}
            }

            if depth == 0 {
                break;
            }
        }

        Ok(())
    }
}
