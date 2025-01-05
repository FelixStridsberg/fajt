use super::Result;
use crate::code_point::CodePoint;
use crate::error::Error;
use crate::token::TokenValue;
use crate::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn read_number_literal(&mut self) -> Result<TokenValue> {
        let current = self.reader.current()?;
        let number_string = match self.reader.peek() {
            Ok('x' | 'X') if current == &'0' => self.read_hex_string()?,
            Ok('o' | 'O') if current == &'0' => self.read_octal_string()?,
            Ok('b' | 'B') if current == &'0' => self.read_binary_string()?,
            Ok('0'..='9') if current == &'0' => {
                let position = self.reader.position();
                return Err(Error::syntax_error(
                    "Zero prefixed numbers are deprecated and not supported".to_owned(),
                    (position, position + 1),
                ));
            }
            _ => self.read_decimal_string()?,
        };

        if self
            .reader
            .current()
            .ok()
            .map_or(false, CodePoint::is_start_of_identifier)
        {
            let position = self.reader.position();
            return Err(Error::syntax_error(
                "Number cannot be followed by identifier without separation".to_owned(),
                (position, position),
            ));
        }

        Ok(literal!(number, number_string))
    }

    fn read_hex_string(&mut self) -> Result<String> {
        let span_start = self.reader.position();

        let mut number_string = String::new();
        number_string.push(self.reader.consume()?); // 0
        number_string.push(self.reader.consume()?); // x or X

        let hex_characters = self.expect_read_number_string(span_start, char::is_ascii_hexdigit)?;
        number_string.push_str(&hex_characters);

        Ok(number_string)
    }

    fn read_octal_string(&mut self) -> Result<String> {
        let span_start = self.reader.position();

        let mut number_string = String::new();
        number_string.push(self.reader.consume()?); // 0
        number_string.push(self.reader.consume()?); // o or O

        let octal_characters =
            self.expect_read_number_string(span_start, |c| ('0'..='7').contains(c))?;
        number_string.push_str(&octal_characters);

        Ok(number_string)
    }

    fn read_binary_string(&mut self) -> Result<String> {
        let span_start = self.reader.position();

        let mut number_string = String::new();
        number_string.push(self.reader.consume()?); // 0
        number_string.push(self.reader.consume()?); // b or B

        let binary_characters =
            self.expect_read_number_string(span_start, |c| c == &'0' || c == &'1')?;
        number_string.push_str(&binary_characters);

        Ok(number_string)
    }

    fn read_decimal_string(&mut self) -> Result<String> {
        let span_start = self.reader.position();

        let mut number_string = String::new();

        let integer_part = self.read_number_string(span_start, char::is_ascii_digit)?;
        number_string.push_str(&integer_part);

        if self.reader.current().ok() == Some(&'.') {
            number_string.push(self.reader.consume()?);

            let fractional_part =
                self.read_number_string(span_start, |c| ('0'..='9').contains(c))?;
            number_string.push_str(&fractional_part);
        }

        if matches!(self.reader.current().ok(), Some(&'e' | &'E')) {
            number_string.push(self.reader.consume()?);

            if matches!(self.reader.current().ok(), Some(&'-' | &'+')) {
                number_string.push(self.reader.consume()?);
            }

            let exponential_part =
                self.expect_read_number_string(span_start, char::is_ascii_digit)?;
            number_string.push_str(&exponential_part);
        }

        Ok(number_string)
    }

    fn expect_read_number_string(
        &mut self,
        span_start: usize,
        check: fn(&char) -> bool,
    ) -> Result<String> {
        let number_string = self.read_number_string(span_start, check)?;

        if number_string.is_empty() {
            let position = self.reader.position();
            return Err(Error::syntax_error(
                "expected number".to_owned(),
                (position, position),
            ));
        }

        Ok(number_string)
    }

    fn read_number_string(
        &mut self,
        span_start: usize,
        check: fn(&char) -> bool,
    ) -> Result<String> {
        let number_str = self.reader.read_while(|c| check(c) || c == &'_')?;
        if number_str.contains("__") {
            let span_end = self.reader.position();
            return Err(Error::syntax_error(
                "number cannot contain multiple adjacent underscores".to_owned(),
                (span_start, span_end),
            ));
        }

        if number_str.ends_with('_') {
            let span_end = self.reader.position();
            return Err(Error::syntax_error(
                "number cannot end with underscore".to_owned(),
                (span_start, span_end),
            ));
        }

        Ok(number_str)
    }
}
