use crate::error::Error;
use crate::error::ErrorKind::EndOfFile;
use crate::token::Base::Decimal;
use crate::token::{AssignOp, Position, ShiftDirection, Token};
use crate::token::{Number, TokenValue};
use std::str::CharIndices;

extern crate macros;

pub mod error;
pub mod token;

type Result<T> = std::result::Result<T, Error>;

struct Reader<'a> {
    input: &'a str,
    iter: CharIndices<'a>,
    current: (usize, char),
    next: Option<(usize, char)>,
    line: u32,
    column: u32,
    end_of_file: bool,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut iter = input.char_indices();
        let current = iter.next().ok_or(Error::of(EndOfFile))?;
        let next = iter.next();

        Ok(Reader {
            input,
            iter,
            current,
            next,
            line: 0,
            column: 0,
            end_of_file: false,
        })
    }

    pub fn position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    pub fn current(&mut self) -> char {
        self.current.1
    }

    pub fn peek(&self) -> Option<char> {
        self.next.map(|(_, c)| c)
    }

    pub fn next(&mut self) -> Result<char> {
        // TODO self.line
        if !self.end_of_file {
            self.column += 1;
        }

        if let Some(next) = self.next {
            self.current = next;
            self.next = self.iter.next();

            Ok(self.current.1)
        } else {
            self.end_of_file = true;
            return Err(Error::of(EndOfFile));
        }
    }
}

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
        if self.reader.end_of_file {
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
                // TODO check if equal sign is current, then it is Assign(LeftShift)
                // TODO check if > is current, then unsight right shift

                match self.reader.current() {
                    '>' => {
                        self.reader.next()?;
                        Ok(TokenValue::BitwiseShift(ShiftDirection::UnsignedRight))
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
        // TODO decimal, octal, hex, etc...

        let mut num_str = String::new();
        num_str.push(self.reader.current());

        loop {
            match self.reader.next() {
                Ok(c) => {
                    if c.is_alphanumeric() {
                        num_str.push(c);
                    } else {
                        break;
                    }
                }
                Err(e) => {
                    if *e.kind() == EndOfFile {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        let value = num_str.parse::<i64>().unwrap(); // TODO error handling
        Ok(TokenValue::Number(Number::Integer(value, Decimal)))
    }

    fn read_identifier_or_keyword(&mut self) -> Result<TokenValue> {
        let mut word = String::new();
        word.push(self.reader.current());

        loop {
            let c = self.reader.next().unwrap(); // TODO
            if c.is_part_of_identifier() {
                word.push(c);
            } else {
                break;
            }
        }

        let value = if let Ok(keyword) = word.parse() {
            TokenValue::Keyword(keyword)
        } else {
            TokenValue::Identifier(word.to_owned())
        };

        Ok(value)
    }
}

trait CodePoint {
    fn is_ecma_whitespace(&self) -> bool;
    fn is_ecma_line_terminator(&self) -> bool;
    fn is_start_of_identifier(&self) -> bool;
    fn is_part_of_identifier(&self) -> bool;
}

impl CodePoint for char {
    fn is_ecma_whitespace(&self) -> bool {
        match self {
            // Per table in ECMA-262
            '\u{0009}' | '\u{000B}' | '\u{000C}' | '\u{0020}' | '\u{00A0}' | '\u{FEFF}' => true,
            // Other Zs
            '\u{1680}' | '\u{2000}'..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}' => true,
            _ => false,
        }
    }

    fn is_ecma_line_terminator(&self) -> bool {
        match self {
            // Per table in ECMA-262
            '\u{000A}' | '\u{000D}' | '\u{2028}' | '\u{2029}' => true,
            _ => false,
        }
    }

    fn is_start_of_identifier(&self) -> bool {
        match self {
            'A'..='Z' | 'a'..='z' | '_' | '$' => true,
            _ => false, // TODO all unicode ID_Start is allowed
                        // TODO unicode escape sequence is allowed (ecma-262: 11.8.4)
        }
    }

    fn is_part_of_identifier(&self) -> bool {
        match self {
            '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' | '$' => true,
            _ => false, // TODO all unicode ID_Continue is allowed
                        // TODO unicode escape sequence is allowed (ecma-262: 11.8.4)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Base::Decimal;
    use crate::token::Keyword::{Const, Let, Var};
    use crate::token::Number::Integer;
    use crate::token::Token;
    use crate::token::TokenValue::{Assign, BitwiseShift, Identifier, Keyword, Number};
    use crate::token::{AssignOp, ShiftDirection};
    use crate::Lexer;

    macro_rules! assert_lexer(
        (input: $input:expr, output: [$(($token:expr, ($col1:expr, $col2:expr)),)*]) => {
            let mut lexer = Lexer::new($input).expect("Could not create lexer, empty input?");
            let tokens = lexer.read().unwrap();

            assert_eq!(vec![$(Token::new($token, ((0, $col1), (0, $col2)))),*], tokens);
        }
    );

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
    fn lex_assignment_bitwise_shift_left() {
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
    fn lex_assignment_bitwise_shift_right() {
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
    fn lex_assignment_bitwise_unsigned_shift_right() {
        assert_lexer!(
            input: "a >>> 10",
            output: [
                (Identifier("a".to_owned()), (0, 1)),
                (BitwiseShift(ShiftDirection::UnsignedRight), (2, 5)),
                (Number(Integer(10, Decimal)), (6, 8)),
            ]
        );
    }
}
