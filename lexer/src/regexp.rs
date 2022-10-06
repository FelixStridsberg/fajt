use fajt_ast::Literal;
use crate::{CodePoint, Lexer, TokenValue};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

impl<'a> Lexer<'a> {
    pub(super) fn read_regexp_literal(&mut self) -> Result<TokenValue> {
        let mut result = String::new();
        let regexp_start = self.reader.consume()?;
        debug_assert_eq!(regexp_start, '/');

        result.push(regexp_start);

        loop {
            let c = self.reader.consume()?;
            result.push(c);

            match c {
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
