use crate::code_point::CodePoint;
use crate::error::Error;
use crate::Lexer;

type Result<T> = std::result::Result<T, Error>;

impl<'a> Lexer<'a> {
    pub(super) fn read_and_expand_unicode_escape_sequence(
        &mut self,
        is_start: bool,
    ) -> Result<char> {
        let span_start = self.reader.position();
        let char = self.read_unicode_escape_sequence()?;
        let valid = if is_start {
            char.is_start_of_identifier()
        } else {
            char.is_part_of_identifier()
        };

        if !valid {
            let span_end = self.reader.position();
            return Err(Error::syntax_error(
                "invalid escape sequence".to_owned(),
                (span_start, span_end),
            ));
        }

        Ok(char)
    }

    fn read_unicode_escape_sequence(&mut self) -> Result<char> {
        let span_start = self.reader.position();

        let slash = self.reader.consume()?;
        debug_assert_eq!(slash, '\\');

        if self.reader.consume()? != 'u' {
            let span_end = self.reader.position();
            return Err(Error::syntax_error(
                "invalid escape sequence".to_owned(),
                (span_start, span_end),
            ));
        }

        let code_point = match self.reader.current()? {
            '{' => self.read_code_point(span_start)?,
            _ => self.read_4digit_hex(span_start)?,
        };

        Ok(char::try_from(code_point).unwrap())
    }

    fn read_code_point(&mut self, span_start: usize) -> Result<u32> {
        let open_bracket = self.reader.consume()?;
        debug_assert_eq!(open_bracket, '{');

        let mut hex = String::with_capacity(4);
        loop {
            hex.push(self.read_hex_char(span_start)?);

            if self.reader.current()? == &'}' {
                self.reader.consume()?;
                break;
            }
        }

        match u32::from_str_radix(&hex, 16) {
            Ok(code_point) if code_point <= 0x10FFFF => Ok(code_point),
            _ => {
                let span_end = self.reader.position();
                Err(Error::syntax_error(
                    "invalid escape sequence".to_owned(),
                    (span_start, span_end),
                ))
            }
        }
    }

    fn read_4digit_hex(&mut self, span_start: usize) -> Result<u32> {
        let mut hex = String::with_capacity(4);
        hex.push(self.read_hex_char(span_start)?);
        hex.push(self.read_hex_char(span_start)?);
        hex.push(self.read_hex_char(span_start)?);
        hex.push(self.read_hex_char(span_start)?);

        // This unwrap is safe because we know the string contains only 4 hex chars.
        Ok(u32::from_str_radix(&hex, 16).unwrap())
    }

    fn read_hex_char(&mut self, span_start: usize) -> Result<char> {
        match self.reader.consume()? {
            c @ ('0'..='9' | 'a'..='f' | 'A'..='F') => Ok(c),
            _ => {
                let span_end = self.reader.position();
                Err(Error::syntax_error(
                    "invalid escape sequence".to_owned(),
                    (span_start, span_end),
                ))
            }
        }
    }
}
