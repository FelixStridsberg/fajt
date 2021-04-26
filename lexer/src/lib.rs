extern crate macros;

mod code_point;
pub mod error;
mod reader;

#[macro_use]
pub mod token;

use crate::code_point::CodePoint;
use crate::error::Error;
use crate::error::ErrorKind::EndOfFile;
use crate::reader::Reader;
use crate::token::Base::{Binary, Decimal, Hex, Octal};
use crate::token::Token;
use crate::token::TokenValue;

type Result<T> = std::result::Result<T, Error>;

pub struct Lexer<'a> {
    reader: Reader<'a>,
}

macro_rules! consume {
    ($self:ident, 1, $produce:expr) => {{
        $self.reader.consume()?;
        Ok($produce)
    }};
    ($self:ident, 2, $produce:expr) => {{
        $self.reader.consume()?;
        $self.reader.consume()?;
        Ok($produce)
    }};
    ($self:ident, 3, $produce:expr) => {{
        $self.reader.consume()?;
        $self.reader.consume()?;
        $self.reader.consume()?;
        Ok($produce)
    }};
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Result<Self> {
        let reader = Reader::new(data)?;
        Ok(Lexer { reader })
    }

    fn skip_whitespaces(&mut self) -> Result<usize> {
        let mut count = 0;

        while self.reader.current().is_ecma_whitespace() {
            count += 1;
            self.reader.next()?;
        }

        Ok(count)
    }

    pub fn read_all(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            match self.read() {
                Ok(token) => tokens.push(token),
                Err(e) => {
                    if *e.kind() != EndOfFile {
                        return Err(e);
                    }
                    break;
                }
            }
        }

        Ok(tokens)
    }

    pub fn read(&mut self) -> Result<Token> {
        if self.reader.eof() {
            return Err(Error::of(EndOfFile));
        }

        self.skip_whitespaces()?;

        let current = self.reader.current();

        let start = self.reader.position();
        let value = match current {
            '=' if self.reader.peek() != Some('=') => {
                self.reader.consume()?;
                Ok(punct!("="))
            }
            // TokenValue::Punct with operator: <op>=
            '/' | '*' | '%' | '+' | '-' | '|' | '^' | '&' if self.reader.peek() == Some('=') => {
                match current {
                    '/' => consume!(self, 2, punct!("/=")),
                    '*' => consume!(self, 2, punct!("*=")),
                    '%' => consume!(self, 2, punct!("%=")),
                    '+' => consume!(self, 2, punct!("+=")),
                    '-' => consume!(self, 2, punct!("-=")),
                    '|' => consume!(self, 2, punct!("|=")),
                    '^' => consume!(self, 2, punct!("^=")),
                    '&' => consume!(self, 2, punct!("&=")),
                    _ => unreachable!(),
                }
            }
            '+' => {
                // TODO check for ++, this can be unary as well
                consume!(self, 1, punct!("+"))
            }
            '-' => {
                // TODO check for --, this can be unary as well
                consume!(self, 1, punct!("-"))
            }
            '*' => match self.reader.peek() {
                Some('*') => {
                    self.reader.consume()?;
                    if self.reader.peek() == Some('=') {
                        consume!(self, 2, punct!("**="))
                    } else {
                        consume!(self, 1, punct!("**"))
                    }
                }
                _ => consume!(self, 1, punct!("*")),
            },
            '/' => {
                // TODO handle comment //
                consume!(self, 1, punct!("/"))
            }
            '&' => {
                if self.reader.peek() == Some('&') {
                    consume!(self, 2, punct!("&&"))
                } else {
                    consume!(self, 1, punct!("&"))
                }
            }
            '|' => {
                if self.reader.peek() == Some('|') {
                    consume!(self, 2, punct!("||"))
                } else {
                    consume!(self, 1, punct!("|"))
                }
            }
            '?' if self.reader.peek() == Some('?') => {
                consume!(self, 2, punct!("??"))
            }
            '<' if self.reader.peek() == Some('<') => {
                self.reader.consume()?;

                if self.reader.peek() == Some('=') {
                    consume!(self, 2, punct!("<<="))
                } else {
                    consume!(self, 1, punct!("<<"))
                }
            }
            '>' if self.reader.peek() == Some('>') => {
                self.reader.consume()?;

                match self.reader.peek() {
                    Some('>') => {
                        self.reader.consume()?;
                        if self.reader.peek() == Some('=') {
                            consume!(self, 2, punct!(">>>="))
                        } else {
                            consume!(self, 1, punct!(">>>"))
                        }
                    }
                    Some('=') => consume!(self, 2, punct!(">>=")),
                    _ => consume!(self, 1, punct!(">>")),
                }
            }
            '0'..='9' => self.read_number_literal(),
            '^' => consume!(self, 1, punct!("^")),
            '%' => consume!(self, 1, punct!("%")),
            ';' => consume!(self, 1, punct!(";")),
            '{' => consume!(self, 1, punct!("{")),
            '}' => consume!(self, 1, punct!("}")),
            ',' => consume!(self, 1, punct!(",")),
            c if c.is_start_of_identifier() => self.read_identifier_or_keyword(),
            c => unimplemented!("Unimplemented: {}", c),
        }?;
        let end = self.reader.position();

        Ok(Token::new(value, (start, end)))
    }

    fn read_number_literal(&mut self) -> Result<TokenValue> {
        let current = self.reader.current();
        let (base, number) = match self.reader.peek() {
            Some('x') | Some('X') if current == '0' => {
                (Hex, self.read_number(16, |c| c.is_ascii_hexdigit())?)
            }
            Some('o') | Some('O') if current == '0' => {
                (Octal, self.read_number(8, |c| ('0'..='7').contains(&c))?)
            }
            Some('b') | Some('B') if current == '0' => {
                (Binary, self.read_number(2, |c| c == '0' || c == '1')?)
            }
            _ => (Decimal, self.read_number(10, char::is_numeric)?),
        };

        Ok(literal!(number, base, number))
    }

    fn read_number(&mut self, base: u32, check: fn(char) -> bool) -> Result<i64> {
        // All but base 10 have 2 char prefix: 0b, 0o, 0x
        if base != 10 {
            self.reader.consume()?;
            self.reader.consume()?;
        }

        let number_str = self.reader.read_until(check)?;

        // The check must be strict enough for a safe unwrap here
        Ok(i64::from_str_radix(&number_str, base).unwrap())
    }

    fn read_identifier_or_keyword(&mut self) -> Result<TokenValue> {
        let word = self
            .reader
            .read_until(|c| char::is_part_of_identifier(&c))?;
        let value = if let Ok(keyword) = word.parse() {
            TokenValue::Keyword(keyword)
        } else {
            TokenValue::Identifier(word)
        };

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Base::Binary;
    use crate::token::Literal::Number;
    use crate::token::Number::Integer;
    use crate::token::Token;
    use crate::token::TokenValue::{Identifier, Literal};
    use crate::Lexer;

    macro_rules! assert_lexer {
        (input: $input:expr, output: [$(($token:expr, ($col1:expr, $col2:expr)),)*]) => {
            let mut lexer = Lexer::new($input).expect("Could not create lexer, empty input?");
            let tokens = lexer.read_all().unwrap();

            assert_eq!(tokens, vec![$(Token::new($token, ($col1, $col2))),*]);
        };
        (input: $input:expr, error: $error:expr) => {
            let mut lexer = Lexer::new($input).expect("Could not create lexer, empty input?");
            let error = lexer.read().expect_err("Expected error but test passed.");

            assert_eq!(error, $error);
        };
    }

    #[test]
    fn lex_number_decimal() {
        assert_lexer!(
            input: "1234",
            output: [
                (literal!(integer, 1234), (0, 4)),
            ]
        );
    }

    #[test]
    fn lex_number_hex() {
        assert_lexer!(
            input: "0xff08",
            output: [
                (literal!(hex, 0xff08), (0, 6)),
            ]
        );
    }

    #[test]
    fn lex_number_octal() {
        assert_lexer!(
            input: "0o347",
            output: [
                (literal!(octal, 0o347), (0, 5)),
            ]
        );
    }

    #[test]
    fn lex_number_binary() {
        assert_lexer!(
            input: "0b10111100",
            output: [
                (Literal(Number(Integer(0b10111100, Binary))), (0, 10)),
            ]
        );
    }

    #[test]
    fn lex_expression_add() {
        assert_lexer!(
            input: "1 + 1",
            output: [
                (literal!(integer, 1), (0, 1)),
                (punct!("+"), (2, 3)),
                (literal!(integer, 1), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_subtract() {
        assert_lexer!(
            input: "1 - 1",
            output: [
                (literal!(integer, 1), (0, 1)),
                (punct!("-"), (2, 3)),
                (literal!(integer, 1), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_multiply() {
        assert_lexer!(
            input: "1 * 1",
            output: [
                (literal!(integer, 1), (0, 1)),
                (punct!("*"), (2, 3)),
                (literal!(integer, 1), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_exponent() {
        assert_lexer!(
            input: "1 ** 1",
            output: [
                (literal!(integer, 1), (0, 1)),
                (punct!("**"), (2, 4)),
                (literal!(integer, 1), (5, 6)),
            ]
        );
    }

    #[test]
    fn lex_expression_divide() {
        assert_lexer!(
            input: "1 / 1",
            output: [
                (literal!(integer, 1), (0, 1)),
                (punct!("/"), (2, 3)),
                (literal!(integer, 1), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_modulus() {
        assert_lexer!(
            input: "1 % 1",
            output: [
                (literal!(integer, 1), (0, 1)),
                (punct!("%"), (2, 3)),
                (literal!(integer, 1), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_and() {
        assert_lexer!(
            input: "1 & 1",
            output: [
                (literal!(integer, 1), (0, 1)),
                (punct!("&"), (2, 3)),
                (literal!(integer, 1), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_or() {
        assert_lexer!(
            input: "1 | 1",
            output: [
                (literal!(integer, 1), (0, 1)),
                (punct!("|"), (2, 3)),
                (literal!(integer, 1), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_xor() {
        assert_lexer!(
            input: "1 ^ 1",
            output: [
                (literal!(integer, 1), (0, 1)),
                (punct!("^"), (2, 3)),
                (literal!(integer, 1), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_shift_left() {
        assert_lexer!(
            input: "a << 10",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (punct!("<<"), (2, 4)),
                (literal!(integer, 10), (5, 7)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_shift_right() {
        assert_lexer!(
            input: "a >> 10",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (punct!(">>"), (2, 4)),
                (literal!(integer, 10), (5, 7)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_unsigned_shift_right() {
        assert_lexer!(
            input: "a >>> 10",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (punct!(">>>"), (2, 5)),
                (literal!(integer, 10), (6, 8)),
            ]
        );
    }

    #[test]
    fn lex_expression_and() {
        assert_lexer!(
            input: "a && b",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (punct!("&&"), (2, 4)),
                (Identifier("b".to_owned()), (5, 6)),
            ]
        );
    }

    #[test]
    fn lex_expression_or() {
        assert_lexer!(
            input: "a || b",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (punct!("||"), (2, 4)),
                (Identifier("b".to_owned()), (5, 6)),
            ]
        );
    }

    #[test]
    fn lex_expression_coalesce() {
        assert_lexer!(
            input: "a ?? b",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (punct!("??"), (2, 4)),
                (Identifier("b".to_owned()), (5, 6)),
            ]
        );
    }

    #[test]
    fn lex_assignment_const() {
        assert_lexer!(
            input: "const variable = 1;",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("="), (15, 16)),
                (literal!(integer, 1), (17, 18)),
                (punct!(";"), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_let() {
        assert_lexer!(
            input: "let variable = 1",
            output: [
                (keyword!("let"), (0, 3)),
                (Identifier("variable".to_owned()), (4, 12)),
                (punct!("="), (13, 14)),
                (literal!(integer, 1), (15, 16)),
            ]
        );
    }

    #[test]
    fn lex_assignment_var() {
        assert_lexer!(
            input: "var variable = 1",
            output: [
                (keyword!("var"), (0, 3)),
                (Identifier("variable".to_owned()), (4, 12)),
                (punct!("="), (13, 14)),
                (literal!(integer, 1), (15, 16)),
            ]
        );
    }

    #[test]
    fn lex_assignment_multiply() {
        assert_lexer!(
            input: "const variable *= 1",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("*="), (15, 17)),
                (literal!(integer, 1), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_exponent() {
        assert_lexer!(
            input: "const variable **= 1",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("**="), (15, 18)),
                (literal!(integer, 1), (19, 20)),
            ]
        );
    }

    #[test]
    fn lex_assignment_divide() {
        assert_lexer!(
            input: "const variable /= 1;",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("/="), (15, 17)),
                (literal!(integer, 1), (18, 19)),
                (punct!(";"), (19, 20)),
            ]
        );
    }

    #[test]
    fn lex_assignment_mod() {
        assert_lexer!(
            input: "const variable %= 1",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("%="), (15, 17)),
                (literal!(integer, 1), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_add() {
        assert_lexer!(
            input: "const variable += 1;",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("+="), (15, 17)),
                (literal!(integer, 1), (18, 19)),
                (punct!(";"), (19, 20)),
            ]
        );
    }

    #[test]
    fn lex_assignment_subtract() {
        assert_lexer!(
            input: "const variable -= 1",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("-="), (15, 17)),
                (literal!(integer, 1), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_and() {
        assert_lexer!(
            input: "const variable &= 1;",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("&="), (15, 17)),
                (literal!(integer, 1), (18, 19)),
                (punct!(";"), (19, 20)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_xor() {
        assert_lexer!(
            input: "const variable ^= 1;",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("^="), (15, 17)),
                (literal!(integer, 1), (18, 19)),
                (punct!(";"), (19, 20)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_or() {
        assert_lexer!(
            input: "const variable |= 1;",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("|="), (15, 17)),
                (literal!(integer, 1), (18, 19)),
                (punct!(";"), (19, 20)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_left_shift() {
        assert_lexer!(
            input: "const variable <<= 1;",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!("<<="), (15, 18)),
                (literal!(integer, 1), (19, 20)),
                (punct!(";"), (20, 21)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_right_shift() {
        assert_lexer!(
            input: "const variable >>= 1;",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!(">>="), (15, 18)),
                (literal!(integer, 1), (19, 20)),
                (punct!(";"), (20, 21)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_unsigned_right_shift() {
        assert_lexer!(
            input: "const variable >>>= 1;",
            output: [
                (keyword!("const"), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (punct!(">>>="), (15, 19)),
                (literal!(integer, 1), (20, 21)),
                (punct!(";"), (21, 22)),
            ]
        );
    }
}
