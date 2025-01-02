use crate::error::Error;
use crate::{CodePoint, Lexer, TokenValue};
use fajt_ast::Literal;

type Result<T> = std::result::Result<T, Error>;

impl Lexer<'_> {
    pub(super) fn read_regexp_literal(&mut self) -> Result<TokenValue> {
        let span_start = self.reader.position();
        let mut result = String::new();
        let regexp_start = self.reader.consume()?;
        debug_assert_eq!(regexp_start, '/');

        result.push(regexp_start);

        loop {
            let c = self.reader.consume()?;
            result.push(c);

            match c {
                '\n' => {
                    let span_end = self.reader.position();
                    return Err(Error::syntax_error(
                        "unterminated regular expression literal".to_owned(),
                        (span_start, span_end - 1),
                    ));
                }
                '/' => break,
                '\\' => result.push(self.reader.consume()?),
                '[' => result.push_str(&self.read_regexp_group_body()?),
                _ => {}
            }
        }

        let flags = self.reader.read_while(char::is_part_of_identifier)?;
        result.push_str(&flags);

        Ok(TokenValue::Literal(Literal::Regexp(result)))
    }

    fn read_regexp_group_body(&mut self) -> Result<String> {
        let mut result = String::new();
        loop {
            let c = self.reader.consume()?;
            result.push(c);

            match c {
                ']' => break,
                '\\' => result.push(self.reader.consume()?),
                _ => {}
            }
        }

        Ok(result)
    }
}
