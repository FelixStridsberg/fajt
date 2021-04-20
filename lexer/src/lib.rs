extern crate macros;

mod code_point;
pub mod error;
mod reader;
pub mod token;

use crate::code_point::CodePoint;
use crate::error::Error;
use crate::error::ErrorKind::EndOfFile;
use crate::reader::Reader;
use crate::token::Base::{Binary, Decimal, Hex, Octal};
use crate::token::{AssignOp, BinaryOp, ShiftDirection, Token};
use crate::token::{Number, TokenValue};

type Result<T> = std::result::Result<T, Error>;

struct Lexer<'a> {
    reader: Reader<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Result<Self> {
        let reader = Reader::new(data)?;
        Ok(Lexer { reader })
    }

    fn skip_whitespaces(&mut self) -> Result<()> {
        // TODO handle semi colon, skipping for now
        while self.reader.current().is_ecma_whitespace() || self.reader.current() == ';' {
            self.reader.next()?;
        }

        Ok(())
    }

    pub fn read(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            match self.next() {
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

    pub fn next(&mut self) -> Result<Token> {
        if self.reader.eof() {
            return Err(Error::of(EndOfFile));
        }

        self.skip_whitespaces()?;

        let current = self.reader.current();

        let start = self.reader.position();
        let value = match current {
            '=' if self.reader.peek() != Some('=') => {
                self.reader.next()?;
                Ok(TokenValue::Assign(AssignOp::None))
            }
            // Assign with operator: <op>=
            '/' | '*' | '%' | '+' | '-' | '|' | '^' | '&' if self.reader.peek() == Some('=') => {
                self.reader.next()?;
                self.reader.next()?;

                match current {
                    '/' => Ok(TokenValue::Assign(AssignOp::Divide)),
                    '*' => Ok(TokenValue::Assign(AssignOp::Multiply)),
                    '%' => Ok(TokenValue::Assign(AssignOp::Modulus)),
                    '+' => Ok(TokenValue::Assign(AssignOp::Add)),
                    '-' => Ok(TokenValue::Assign(AssignOp::Subtract)),
                    '|' => Ok(TokenValue::Assign(AssignOp::BitwiseOr)),
                    '^' => Ok(TokenValue::Assign(AssignOp::BitwiseXOr)),
                    '&' => Ok(TokenValue::Assign(AssignOp::BitwiseAnd)),
                    _ => unreachable!(),
                }
            }
            '+' => {
                // TODO check for ++, this can be unary as well
                self.reader.next()?;
                Ok(TokenValue::BinaryOperator(BinaryOp::Add))
            }
            '-' => {
                // TODO check for --, this can be unary as well
                self.reader.next()?;
                Ok(TokenValue::BinaryOperator(BinaryOp::Subtract))
            }
            '%' => {
                self.reader.next()?;
                Ok(TokenValue::BinaryOperator(BinaryOp::Modulus))
            }
            '*' => match self.reader.peek() {
                Some('*') => {
                    self.reader.next()?;
                    self.reader.next()?;

                    if self.reader.current() == '=' {
                        self.reader.next()?;
                        Ok(TokenValue::Assign(AssignOp::Exponent))
                    } else {
                        Ok(TokenValue::BinaryOperator(BinaryOp::Exponent))
                    }
                }
                _ => {
                    self.reader.next()?;
                    Ok(TokenValue::BinaryOperator(BinaryOp::Multiply))
                }
            },
            '/' => {
                self.reader.next()?;
                Ok(TokenValue::BinaryOperator(BinaryOp::Divide))
            }
            '&' => {
                self.reader.next()?;
                if self.reader.current() == '&' {
                    self.reader.next()?;
                    Ok(TokenValue::BinaryOperator(BinaryOp::LogicalAnd))
                } else {
                    Ok(TokenValue::BinaryOperator(BinaryOp::BitwiseAnd))
                }
            }
            '|' => {
                self.reader.next()?;
                if self.reader.current() == '|' {
                    self.reader.next()?;
                    Ok(TokenValue::BinaryOperator(BinaryOp::LogicalOr))
                } else {
                    Ok(TokenValue::BinaryOperator(BinaryOp::BitwiseOr))
                }
            }
            '?' if self.reader.peek() == Some('?') => {
                self.reader.next()?;
                self.reader.next()?;
                Ok(TokenValue::BinaryOperator(BinaryOp::Coalesce))
            }
            '^' => {
                self.reader.next()?;
                Ok(TokenValue::BinaryOperator(BinaryOp::BitwiseXOr))
            }
            '<' if self.reader.peek() == Some('<') => {
                self.reader.next()?;
                self.reader.next()?;

                if self.reader.current() == '=' {
                    self.reader.next()?;
                    Ok(TokenValue::Assign(AssignOp::LeftShift))
                } else {
                    Ok(TokenValue::BitwiseShift(ShiftDirection::Left))
                }
            }
            '>' if self.reader.peek() == Some('>') => {
                self.reader.next()?;
                self.reader.next()?;

                match self.reader.current() {
                    '>' => {
                        self.reader.next()?;
                        if self.reader.current() == '=' {
                            self.reader.next()?;
                            Ok(TokenValue::Assign(AssignOp::URightShift))
                        } else {
                            Ok(TokenValue::BitwiseShift(ShiftDirection::UnsignedRight))
                        }
                    }
                    '=' => {
                        self.reader.next()?;
                        Ok(TokenValue::Assign(AssignOp::RightShift))
                    }
                    _ => Ok(TokenValue::BitwiseShift(ShiftDirection::Right)),
                }
            }
            '0'..='9' => self.read_number(),
            c if c.is_start_of_identifier() => self.read_identifier_or_keyword(),
            c => unimplemented!("Unimplemented: {}", c),
        }?;
        let end = self.reader.position();

        Ok(Token::new(value, (start, end)))
    }

    fn read_number(&mut self) -> Result<TokenValue> {
        match self.reader.peek() {
            Some('x') | Some('X') => self.read_hex(),
            Some('o') | Some('O') => self.read_octal(),
            Some('b') | Some('B') => self.read_binary(),
            _ => self.read_decimal(),
        }
    }

    fn read_decimal(&mut self) -> Result<TokenValue> {
        let num_str = self.reader.read_until(char::is_numeric)?;
        let value = num_str.parse::<i64>().unwrap(); // TODO error handling
        Ok(TokenValue::Number(Number::Integer(value, Decimal)))
    }

    fn read_hex(&mut self) -> Result<TokenValue> {
        self.reader.next()?; // 0
        self.reader.next()?; // x

        let hex_str = self.reader.read_until(|c| c.is_ascii_hexdigit())?;
        let value = i64::from_str_radix(&hex_str, 16).unwrap();

        Ok(TokenValue::Number(Number::Integer(value, Hex)))
    }

    fn read_octal(&mut self) -> Result<TokenValue> {
        self.reader.next()?; // 0
        self.reader.next()?; // o

        let octal_str = self.reader.read_until(|c| c >= '0' && c <= '7')?;
        let value = i64::from_str_radix(&octal_str, 8).unwrap();

        Ok(TokenValue::Number(Number::Integer(value, Octal)))
    }

    fn read_binary(&mut self) -> Result<TokenValue> {
        self.reader.next()?; // 0
        self.reader.next()?; // b

        let octal_str = self.reader.read_until(|c| c == '0' || c == '1')?;
        let value = i64::from_str_radix(&octal_str, 2).unwrap();

        Ok(TokenValue::Number(Number::Integer(value, Binary)))
    }

    fn read_identifier_or_keyword(&mut self) -> Result<TokenValue> {
        let word = self.reader.read_until(char::is_part_of_identifier)?;
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
    use crate::token::Base::{Binary, Decimal, Hex, Octal};
    use crate::token::Keyword::{Const, Let, Var};
    use crate::token::Number::Integer;
    use crate::token::Token;
    use crate::token::TokenValue::{
        Assign, BinaryOperator, BitwiseShift, Identifier, Keyword, Number,
    };
    use crate::token::{AssignOp, BinaryOp, ShiftDirection};
    use crate::Lexer;

    macro_rules! assert_lexer(
        (input: $input:expr, output: [$(($token:expr, ($col1:expr, $col2:expr)),)*]) => {
            let mut lexer = Lexer::new($input).expect("Could not create lexer, empty input?");
            let tokens = lexer.read().unwrap();

            assert_eq!(vec![$(Token::new($token, ((0, $col1), (0, $col2)))),*], tokens);
        }
    );

    #[test]
    fn lex_number_decimal() {
        assert_lexer!(
            input: "1234",
            output: [
                (Number(Integer(1234, Decimal)), (0, 4)),
            ]
        );
    }

    #[test]
    fn lex_number_hex() {
        assert_lexer!(
            input: "0xff08",
            output: [
                (Number(Integer(0xff08, Hex)), (0, 6)),
            ]
        );
    }

    #[test]
    fn lex_number_octal() {
        assert_lexer!(
            input: "0o347",
            output: [
                (Number(Integer(0o347, Octal)), (0, 5)),
            ]
        );
    }

    #[test]
    fn lex_number_binary() {
        assert_lexer!(
            input: "0b10111100",
            output: [
                (Number(Integer(0b10111100, Binary)), (0, 10)),
            ]
        );
    }

    #[test]
    fn lex_expression_add() {
        assert_lexer!(
            input: "1 + 1",
            output: [
                (Number(Integer(1, Decimal)), (0, 1)),
                (BinaryOperator(BinaryOp::Add), (2, 3)),
                (Number(Integer(1, Decimal)), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_subtract() {
        assert_lexer!(
            input: "1 - 1",
            output: [
                (Number(Integer(1, Decimal)), (0, 1)),
                (BinaryOperator(BinaryOp::Subtract), (2, 3)),
                (Number(Integer(1, Decimal)), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_multiply() {
        assert_lexer!(
            input: "1 * 1",
            output: [
                (Number(Integer(1, Decimal)), (0, 1)),
                (BinaryOperator(BinaryOp::Multiply), (2, 3)),
                (Number(Integer(1, Decimal)), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_exponent() {
        assert_lexer!(
            input: "1 ** 1",
            output: [
                (Number(Integer(1, Decimal)), (0, 1)),
                (BinaryOperator(BinaryOp::Exponent), (2, 4)),
                (Number(Integer(1, Decimal)), (5, 6)),
            ]
        );
    }

    #[test]
    fn lex_expression_divide() {
        assert_lexer!(
            input: "1 / 1",
            output: [
                (Number(Integer(1, Decimal)), (0, 1)),
                (BinaryOperator(BinaryOp::Divide), (2, 3)),
                (Number(Integer(1, Decimal)), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_modulus() {
        assert_lexer!(
            input: "1 % 1",
            output: [
                (Number(Integer(1, Decimal)), (0, 1)),
                (BinaryOperator(BinaryOp::Modulus), (2, 3)),
                (Number(Integer(1, Decimal)), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_and() {
        assert_lexer!(
            input: "1 & 1",
            output: [
                (Number(Integer(1, Decimal)), (0, 1)),
                (BinaryOperator(BinaryOp::BitwiseAnd), (2, 3)),
                (Number(Integer(1, Decimal)), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_or() {
        assert_lexer!(
            input: "1 | 1",
            output: [
                (Number(Integer(1, Decimal)), (0, 1)),
                (BinaryOperator(BinaryOp::BitwiseOr), (2, 3)),
                (Number(Integer(1, Decimal)), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_xor() {
        assert_lexer!(
            input: "1 ^ 1",
            output: [
                (Number(Integer(1, Decimal)), (0, 1)),
                (BinaryOperator(BinaryOp::BitwiseXOr), (2, 3)),
                (Number(Integer(1, Decimal)), (4, 5)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_shift_left() {
        assert_lexer!(
            input: "a << 10",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (BitwiseShift(ShiftDirection::Left), (2, 4)),
                (Number(Integer(10, Decimal)), (5, 7)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_shift_right() {
        assert_lexer!(
            input: "a >> 10",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (BitwiseShift(ShiftDirection::Right), (2, 4)),
                (Number(Integer(10, Decimal)), (5, 7)),
            ]
        );
    }

    #[test]
    fn lex_expression_bitwise_unsigned_shift_right() {
        assert_lexer!(
            input: "a >>> 10",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (BitwiseShift(ShiftDirection::UnsignedRight), (2, 5)),
                (Number(Integer(10, Decimal)), (6, 8)),
            ]
        );
    }

    #[test]
    fn lex_expression_and() {
        assert_lexer!(
            input: "a && b;",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (BinaryOperator(BinaryOp::LogicalAnd), (2, 4)),
                (Identifier("b".to_owned()), (5, 6)),
            ]
        );
    }

    #[test]
    fn lex_expression_or() {
        assert_lexer!(
            input: "a || b;",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (BinaryOperator(BinaryOp::LogicalOr), (2, 4)),
                (Identifier("b".to_owned()), (5, 6)),
            ]
        );
    }

    #[test]
    fn lex_expression_coalesce() {
        assert_lexer!(
            input: "a ?? b;",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (BinaryOperator(BinaryOp::Coalesce), (2, 4)),
                (Identifier("b".to_owned()), (5, 6)),
            ]
        );
    }

    #[test]
    fn lex_assignment_const() {
        assert_lexer!(
            input: "const variable = 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::None), (15, 16)),
                (Number(Integer(1, Decimal)), (17, 18)),
            ]
        );
    }

    #[test]
    fn lex_assignment_let() {
        assert_lexer!(
            input: "let variable = 1;",
            output: [
                (Keyword(Let), (0, 3)),
                (Identifier("variable".to_owned()), (4, 12)),
                (Assign(AssignOp::None), (13, 14)),
                (Number(Integer(1, Decimal)), (15, 16)),
            ]
        );
    }

    #[test]
    fn lex_assignment_var() {
        assert_lexer!(
            input: "var variable = 1;",
            output: [
                (Keyword(Var), (0, 3)),
                (Identifier("variable".to_owned()), (4, 12)),
                (Assign(AssignOp::None), (13, 14)),
                (Number(Integer(1, Decimal)), (15, 16)),
            ]
        );
    }

    #[test]
    fn lex_assignment_multiply() {
        assert_lexer!(
            input: "const variable *= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::Multiply), (15, 17)),
                (Number(Integer(1, Decimal)), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_exponent() {
        assert_lexer!(
            input: "const variable **= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::Exponent), (15, 18)),
                (Number(Integer(1, Decimal)), (19, 20)),
            ]
        );
    }

    #[test]
    fn lex_assignment_divide() {
        assert_lexer!(
            input: "const variable /= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::Divide), (15, 17)),
                (Number(Integer(1, Decimal)), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_mod() {
        assert_lexer!(
            input: "const variable %= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::Modulus), (15, 17)),
                (Number(Integer(1, Decimal)), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_add() {
        assert_lexer!(
            input: "const variable += 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::Add), (15, 17)),
                (Number(Integer(1, Decimal)), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_subtract() {
        assert_lexer!(
            input: "const variable -= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::Subtract), (15, 17)),
                (Number(Integer(1, Decimal)), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_and() {
        assert_lexer!(
            input: "const variable &= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::BitwiseAnd), (15, 17)),
                (Number(Integer(1, Decimal)), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_xor() {
        assert_lexer!(
            input: "const variable ^= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::BitwiseXOr), (15, 17)),
                (Number(Integer(1, Decimal)), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_or() {
        assert_lexer!(
            input: "const variable |= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::BitwiseOr), (15, 17)),
                (Number(Integer(1, Decimal)), (18, 19)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_left_shift() {
        assert_lexer!(
            input: "const variable <<= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::LeftShift), (15, 18)),
                (Number(Integer(1, Decimal)), (19, 20)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_right_shift() {
        assert_lexer!(
            input: "const variable >>= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::RightShift), (15, 18)),
                (Number(Integer(1, Decimal)), (19, 20)),
            ]
        );
    }

    #[test]
    fn lex_assignment_bitwise_unsigned_right_shift() {
        assert_lexer!(
            input: "const variable >>>= 1;",
            output: [
                (Keyword(Const), (0, 5)),
                (Identifier("variable".to_owned()), (6, 14)),
                (Assign(AssignOp::URightShift), (15, 19)),
                (Number(Integer(1, Decimal)), (20, 21)),
            ]
        );
    }
}
