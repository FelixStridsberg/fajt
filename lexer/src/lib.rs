use crate::error::Error;
use crate::error::ErrorKind::EndOfFile;
use crate::token::Base::Decimal;
use crate::token::{AssignOp, Position, Token};
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
        })
    }

    pub fn current(&mut self) -> char {
        self.current.1
    }

    pub fn peek(&self) -> Option<char> {
        self.next.map(|(_, c)| c)
    }

    pub fn next(&mut self) -> Result<char> {
        self.current = self.next.ok_or(Error::of(EndOfFile))?;
        self.next = self.iter.next();
        Ok(self.current.1)
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
        self.skip_whitespaces()?;

        let c = self.reader.current();

        Ok(match c {
            c if c.is_start_of_identifier() => self.read_identifier_or_keyword(),
            '=' if self.reader.peek() != Some('=') => {
                self.reader.next()?;

                let start = Position { line: 0, column: 0 };
                let end = Position { line: 0, column: 0 };
                Token::new(TokenValue::Assign(AssignOp::None), (start, end))
            }
            '0'..='9' => self.read_number(),
            c => unimplemented!("Unimplemented: {}", c),
        })
    }

    fn read_number(&mut self) -> Token {
        // TODO decimal, octal, hex, etc...

        let mut num_str = String::new();
        num_str.push(self.reader.current());

        loop {
            let c = self.reader.next().unwrap(); // TODO
            if c.is_alphanumeric() {
                num_str.push(c);
            } else {
                break;
            }
        }

        let value = num_str.parse::<i64>().unwrap(); // TODO error handling

        let start = Position { line: 0, column: 0 };
        let end = Position { line: 0, column: 0 };

        Token::new(
            TokenValue::Number(Number::Integer(value, Decimal)),
            (start, end),
        )
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
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

        let start = Position { line: 0, column: 0 };
        let end = Position { line: 0, column: 0 };

        Token::new(value, (start, end))
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
    use crate::token::AssignOp;
    use crate::token::Base::Decimal;
    use crate::token::Keyword::{Const, Let, Var};
    use crate::token::Number::Integer;
    use crate::token::TokenValue;
    use crate::token::TokenValue::{Assign, Identifier, Keyword, Number};
    use crate::Lexer;

    macro_rules! assert_lexer(
        (input: $input:expr, output: [$($output:expr),*$(,)?]) => {
            let mut lexer = Lexer::new($input).expect("Could not create lexer, empty input?");
            let tokens: Vec<TokenValue> = lexer.read().unwrap().into_iter().map(|t| t.value).collect();

            assert_eq!(vec![$($output),*], tokens);
        }
    );

    #[test]
    fn lex_assignment_const() {
        assert_lexer!(
            input: "const variable = 1;",
            output: [
                Keyword(Const),
                Identifier("variable".to_owned()),
                Assign(AssignOp::None),
                Number(Integer(1, Decimal)),
            ]
        );
    }

    #[test]
    fn lex_assignment_let() {
        assert_lexer!(
            input: "let variable = 1;",
            output: [
                Keyword(Let),
                Identifier("variable".to_owned()),
                Assign(AssignOp::None),
                Number(Integer(1, Decimal)),
            ]
        );
    }

    #[test]
    fn lex_assignment_var() {
        assert_lexer!(
            input: "var variable = 1;",
            output: [
                Keyword(Var),
                Identifier("variable".to_owned()),
                Assign(AssignOp::None),
                Number(Integer(1, Decimal)),
            ]
        );
    }
}
