use crate::error::Error;
use crate::token::Base::Decimal;
use crate::token::Number;
use crate::token::{AssignOp, Keyword, Token};
use std::str::CharIndices;

pub mod error;
pub mod token;

type Result<T> = std::result::Result<T, Error>;

struct Reader<'a> {
    input: &'a str,
    iter: CharIndices<'a>,
    current: Option<(usize, char)>,
    next: Option<(usize, char)>,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut iter = input.char_indices();
        let current = iter.next();
        let next = iter.next();

        Reader {
            input,
            iter,
            current,
            next,
        }
    }

    pub fn current(&mut self) -> char {
        let (_, current) = self.current.unwrap();
        current
    }

    pub fn peek(&self) -> Option<char> {
        self.next.map(|(_, c)| c)
    }

    pub fn next(&mut self) -> Option<char> {
        self.current = self.next;
        self.next = self.iter.next();
        self.current.map(|(_, c)| c)
    }
}

struct Lexer<'a> {
    reader: Reader<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        let reader = Reader::new(data);
        Lexer { reader }
    }

    fn skip_whitespaces(&mut self) {
        // TODO handle semi colon, skipping for now
        while self.reader.current().is_ecma_whitespace() || self.reader.current() == ';' {
            if let None = self.reader.next() {
                break;
            }
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        let c = self.reader.current();

        Some(match c {
            c if c.is_start_of_identifier() => self.read_identifier_or_keyword(),
            '=' if self.reader.peek() != Some('=') => {
                self.reader.next();
                Token::Assign(AssignOp::None)
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

        Token::Number(Number::Integer(num_str.parse::<i64>().unwrap(), Decimal))
        // TODO
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

        if let Some(keyword) = Keyword::from_string(&word) {
            Token::Keyword(keyword)
        } else {
            Token::Identifier(word.to_owned())
        }
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

mod tests {
    use crate::token::AssignOp;
    use crate::token::Base::Decimal;
    use crate::token::Keyword::Const;
    use crate::token::Number::Integer;
    use crate::token::Token::{Assign, Identifier, Keyword, Number};
    use crate::Lexer;

    #[test]
    fn lex_assignment_const() {
        let input = "const variable = 1;";

        let mut lexer = Lexer::new(input);

        assert_eq!(Some(Keyword(Const)), lexer.next());
        assert_eq!(Some(Identifier("variable".to_owned())), lexer.next());
        assert_eq!(Some(Assign(AssignOp::None)), lexer.next());
        assert_eq!(Some(Number(Integer(1, Decimal))), lexer.next());
        assert_eq!(None, lexer.next());
    }
}
