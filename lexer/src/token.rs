use macros::FromString;
use std::fmt::{Display, Formatter};
use std::fmt;

macro_rules! literal(
    (integer, $value:expr) => {
        literal!(crate::token::Base::Decimal, $value)
    };
    (hex, $value:expr) => {
        literal!(crate::token::Base::Hex, $value)
    };
    (octal, $value:expr) => {
        literal!(crate::token::Base::Octal, $value)
    };
    ($type:expr, $value:expr) => {
        crate::token::TokenValue::Literal(
            crate::token::Literal::Number(
                crate::token::Number::Integer(
                    $value, $type
                )
            )
        )
    };
);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, FromString)]
pub enum Keyword {
    Const,
    Let,
    Var,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Base {
    Binary,
    Decimal,
    Hex,
    Octal,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Number {
    Integer(i64, Base),
    Decimal(f64),
}

macro_rules! punct_wrap {
    ($token:path) => { TokenValue::Punct($token) }
}

#[derive(Debug, PartialOrd, PartialEq, FromString)]
#[from_string_macro("punct")]
#[from_string_macro_wrap(punct_wrap)]
pub enum Punct {
    #[from_string("(")]
    ParantOpen,
    #[from_string(")")]
    ParantClose,
    #[from_string("[")]
    BraceOpen,
    #[from_string("]")]
    BraceClose,
    #[from_string("{")]
    BracketOpen,
    #[from_string("}")]
    BracketClose,
    #[from_string(".")]
    Dot,
    #[from_string("...")]
    TripleDot,
    #[from_string(";")]
    SemiColon,
    #[from_string(",")]
    Comma,
    #[from_string("<")]
    LessThan,
    #[from_string("<<")]
    DoubleLessThan,
    #[from_string(">")]
    GreaterThan,
    #[from_string(">>")]
    DoubleGreaterThan,
    #[from_string(">>>")]
    TripleGreaterThan,
    #[from_string("=")]
    Equal,
    #[from_string("==")]
    DoubleEqual,
    #[from_string("<=")]
    LessEqual,
    #[from_string("<<=")]
    DoubleLessEqual,
    #[from_string(">=")]
    GreaterEqual,
    #[from_string(">>=")]
    DoubleGreaterEqual,
    #[from_string(">>>=")]
    TripleGreaterEqual,
    #[from_string("=>")]
    EqualGreater,
    #[from_string("!=")]
    NotEqual,
    #[from_string("+=")]
    PlusEqual,
    #[from_string("-=")]
    MinusEqual,
    #[from_string("*=")]
    StarEqual,
    #[from_string("**=")]
    DoubleStarEqual,
    #[from_string("/=")]
    SlashEqual,
    #[from_string("%=")]
    PercentEqual,
    #[from_string("|=")]
    PipeEqual,
    #[from_string("^=")]
    CaretEqual,
    #[from_string("&=")]
    AmpersandEqual,
    #[from_string("===")]
    TripleEqual,
    #[from_string("!==")]
    ExclamationDoubleEqual,
    #[from_string("+")]
    Plus,
    #[from_string("++")]
    DoublePlus,
    #[from_string("-")]
    Minus,
    #[from_string("--")]
    DoubleMinus,
    #[from_string("*")]
    Star,
    #[from_string("**")]
    DoubleStar,
    #[from_string("/")]
    Slash,
    #[from_string("%")]
    Percent,
    #[from_string("&")]
    Ampersand,
    #[from_string("&&")]
    DoubleAmpersand,
    #[from_string("|")]
    Pipe,
    #[from_string("||")]
    DoublePipe,
    #[from_string("^")]
    Caret,
    #[from_string("!")]
    Exclamation,
    #[from_string("~")]
    Tilde,
    #[from_string("?")]
    QuestionMark,
    #[from_string("??")]
    DoubleQuestionMark,
    #[from_string(":")]
    Colon,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Literal {
    Number(Number)
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum TokenValue {
    Keyword(Keyword),
    Identifier(String),
    Punct(Punct),
    Literal(Literal),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct FilePosition {
    pub line: usize,
    pub column: usize,
}

impl Display for FilePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Into<FilePosition> for (usize, usize) {
    fn into(self) -> FilePosition {
        FilePosition {
            line: self.0,
            column: self.1,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Span {
    pub start: FilePosition,
    pub end: FilePosition,
}

impl<P: Into<FilePosition>> Into<Span> for (P, P) {
    fn into(self) -> Span {
        Span {
            start: self.0.into(),
            end: self.1.into(),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Token {
    pub value: TokenValue,
    pub location: Span,
}

impl Token {
    pub fn new<L: Into<Span>>(value: TokenValue, location: L) -> Self {
        Token {
            value,
            location: location.into(),
        }
    }
}
