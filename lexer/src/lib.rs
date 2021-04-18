use crate::token::{Keyword, Token};
use std::str::CharIndices;

pub mod token;

struct Reader<'a> {
    input: &'a str,
    iter: CharIndices<'a>,
    current: (usize, char),
    next: Option<(usize, char)>,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut iter = input.char_indices();
        let current = iter
            .next()
            .expect("Cannot create a reader with empty input.");
        let next = iter.next();

        Reader {
            input,
            iter,
            current,
            next,
        }
    }

    pub fn current(&mut self) -> char {
        let (_, current) = self.current;
        current
    }

    pub fn next(&mut self) -> Option<char> {
        if let Some(next) = self.next {
            self.current = next;
            self.next = self.iter.next();
            Some(next.1)
        } else {
            None
        }
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
        while self.reader.current().is_ecma_whitespace() {
            self.reader.next();
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        let c = self.reader.current();

        Some(match c {
            c if c.is_start_of_identifier() => self.read_identifier_or_keyword(),
            _ => unimplemented!(),
        })
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

fn read_statement_list() {
    //    read
}

#[cfg(test)]
mod tests {
    /*
    use crate::token::{Keyword, AssignOp};
    use crate::token::Keyword::Const;
    use crate::token::Token::{Assign, Number};
    use crate::token::Number::Integer;
    use crate::token::Base::Decimal;
     */
    use crate::Lexer;

    #[test]
    fn lex_assignment_const() {
        let input = "const variable = 1;";

        let mut lexer = Lexer::new(input);

        println!("Token1: {:?}", lexer.next());
        println!("Token2: {:?}", lexer.next());
        assert_eq!(1, 2);
        /*
        let expect = [
            Keyword(Const),
            Identifier("variable"),
            Assign(AssignOp::None),
            Number(Integer(1, Decimal))
        ];*/
    }
}
