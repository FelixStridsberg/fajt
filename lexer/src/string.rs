use crate::{error::Error, token::TokenValue, Lexer, Result};
use fajt_ast::{LitString, Literal};

impl<'a> Lexer<'a> {
    pub(super) fn read_string_literal(&mut self) -> Result<TokenValue> {
        let delimiter = self.reader.consume()?;
        debug_assert!(delimiter == '"' || delimiter == '\'');

        let mut value = String::new();
        self.read_until_unescaped_delimiter(delimiter, &mut value)?;

        Ok(TokenValue::Literal(Literal::String(LitString {
            value,
            delimiter,
        })))
    }

    /// Consumes from reader and push to `result` until an unescaped `delimiter` is reached.
    fn read_until_unescaped_delimiter(
        &mut self,
        delimiter: char,
        result: &mut String,
    ) -> Result<()> {
        loop {
            let c = self.reader.consume()?;

            if c == '\\' {
                self.read_escape_sequence(result)?;
            } else if c == delimiter {
                break;
            } else {
                self.validate_character(c)?;
                result.push(c);
            }
        }

        Ok(())
    }

    fn read_escape_sequence(&mut self, result: &mut String) -> Result<()> {
        let c = self.reader.consume()?;
        result.push(c);

        if c == '\r' && self.reader.current().ok() == Some(&'\n') {
            result.push(self.reader.consume()?);
        }

        Ok(())
    }

    fn validate_character(&self, char: char) -> Result<()> {
        if char == '\r' || char == '\n' {
            let char_pos = self.reader.position() - 1;
            return Err(Error::syntax_error(
                "String contained unescaped new line".to_owned(),
                (char_pos, char_pos),
            ));
        }

        Ok(())
    }
}
