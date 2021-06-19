use crate::error::{Error, ErrorKind};
use fajt_common::io::PeekRead;
use fajt_macros::FromString;
use std::vec::IntoIter;

#[macro_export]
macro_rules! token_matches {
    ($token:expr, $($value:pat)|+) => {
        matches!($token, $( $crate::token::Token { value: $value, .. } )|+);
    };
    ($token:expr, @ident) => {
        token_matches!($token, $crate::token::TokenValue::Identifier(_))
    };
    ($token:expr, @literal) => {
        token_matches!($token, $crate::token::TokenValue::Literal(_))
    };
    ($value:pat) => {
        $crate::token::Token { value: $value, .. }
    };
    (@ident) => {
        token_matches!($crate::token::TokenValue::Identifier(_))
    };
    (@literal) => {
        token_matches!($crate::token::TokenValue::Literal(_))
    };
    ($token:expr, opt: $value:pat) => {
        matches!($token, Some($crate::token::Token { value: $value, .. }));
    };
    ($token:expr, ok: $($value:pat)|+) => {
        matches!(
            $token,
            $(
                Ok($crate::token::Token { value: $value, .. })
            )|+
        );
    };
    (opt: $value:pat) => {
        Some($crate::token::Token { value: $value, .. })
    };
    (ok: $value:pat) => {
        Ok($crate::token::Token { value: $value, .. })
    };
}

bitflags! {
    /// Some keywords are reserved only in specific contexts.
    /// This represents the different contexts.
    pub struct KeywordContext: u32 {
        const AWAIT  = 0b00000001;
        const YIELD  = 0b00000010;
        const STRICT = 0b10000000;
    }
}

/// When working with tokens, do not use this enum directly. Instead use the macro
/// generated by the`from_string_macro`.
///
/// Example:
/// ```
/// # #[macro_use]
/// # extern crate fajt_lexer;
/// # use fajt_lexer::token::{TokenValue, Keyword};
/// # fn main() {
/// assert_eq!(keyword!("const"), TokenValue::Keyword(Keyword::Const))
/// # }
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, FromString, Clone)]
#[from_string_macro("keyword")]
#[from_string_macro_rules(
    ($variant:ident) => {
        $crate::token::TokenValue::Keyword($crate::token::Keyword::$variant)
    };
)]
pub enum Keyword {
    Await,
    As,
    Async,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    From,
    Function,
    Get,
    If,
    Implements,
    Import,
    In,
    Instanceof,
    Interface,
    Let,
    New,
    Null,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Set,
    Static,
    Super,
    Switch,
    Target,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield,
}

impl Keyword {
    /// Tries to turn a keyword into an identifier string.
    /// Succeeds only if that keyword is not reserved in the provided context.
    pub fn into_identifier_string(self, ctx: KeywordContext) -> Result<String, Error> {
        if self.is_allows_as_identifier(ctx) {
            Ok(self.to_string())
        } else {
            Err(Error::of(ErrorKind::ForbiddenIdentifier(self)))
        }
    }

    /// True if the keyword is allowed to be an identifier in the context provided.
    pub fn is_allows_as_identifier(&self, ctx: KeywordContext) -> bool {
        match self {
            Self::As | Self::Async | Self::From | Self::Get | Self::Set | Self::Target => true,
            Self::Await if !ctx.contains(KeywordContext::AWAIT) => true,
            Self::Yield if !ctx.contains(KeywordContext::YIELD) => true,
            Self::Implements
            | Self::Interface
            | Self::Let
            | Self::Package
            | Self::Private
            | Self::Protected
            | Self::Public
            | Self::Static
                if !ctx.contains(KeywordContext::STRICT) =>
            {
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum Base {
    Binary,
    Decimal,
    Hex,
    Octal,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Number {
    Integer(i64, Base),
    Decimal(f64),
}

/// When working with tokens, do not use this enum directly. Instead use the macro
/// generated by the`from_string_macro`.
///
/// Example:
/// ```
/// # #[macro_use]
/// # extern crate fajt_lexer;
/// # use fajt_lexer::token::{TokenValue, Punct};
/// # fn main() {
/// assert_eq!(punct!("["), TokenValue::Punct(Punct::BraceOpen))
/// # }
/// ```
#[derive(Debug, PartialOrd, PartialEq, FromString, Clone)]
#[from_string_macro("punct")]
#[from_string_macro_rules(
    ($variant:ident) => {
        $crate::token::TokenValue::Punct($crate::token::Punct::$variant)
    };
)]
pub enum Punct {
    #[from_string("(")]
    ParenOpen,
    #[from_string(")")]
    ParenClose,
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

#[macro_export]
macro_rules! literal(
    (integer, $value:expr) => {
        literal!(number, crate::token::Base::Decimal, $value)
    };
    (hex, $value:expr) => {
        literal!(number, crate::token::Base::Hex, $value)
    };
    (octal, $value:expr) => {
        literal!(number, crate::token::Base::Octal, $value)
    };
    (binary, $value:expr) => {
        literal!(number, crate::token::Base::Binary, $value)
    };
    (decimal, $value:expr) => {
         crate::token::TokenValue::Literal(
            crate::token::Literal::Number(
                crate::token::Number::Decimal(
                    $value
                )
            )
        )
    };
    (number, $type:expr, $value:expr) => {
        crate::token::TokenValue::Literal(
            crate::token::Literal::Number(
                crate::token::Number::Integer(
                    $value, $type
                )
            )
        )
    };
    (string, $type:expr, $value:expr) => {
         crate::token::TokenValue::Literal(
            crate::token::Literal::String(
                $value.to_owned(), $type
            )
        )
    }
);

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Literal {
    Number(Number),
    String(String, char),
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum TokenValue {
    Keyword(Keyword),
    Identifier(String),
    Punct(Punct),
    Literal(Literal),
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl From<(usize, usize)> for Span {
    fn from((start, end): (usize, usize)) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Token {
    pub value: TokenValue,
    pub first_on_line: bool,
    pub span: Span,
}

impl Token {
    pub fn new<S: Into<Span>>(value: TokenValue, first_on_line: bool, span: S) -> Self {
        Token {
            value,
            first_on_line,
            span: span.into(),
        }
    }
}

impl PeekRead<Token> for IntoIter<Token> {
    type Error = Error;

    fn next(&mut self) -> std::result::Result<Option<(usize, Token)>, Self::Error> {
        if let Some(token) = Iterator::next(self) {
            Ok(Some((token.span.end, token)))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{Error, ErrorKind};
    use crate::token::{Keyword, KeywordContext};

    #[test]
    fn keyword_into_identifier() {
        let identifier = Keyword::Async
            .into_identifier_string(KeywordContext::empty())
            .unwrap();
        assert_eq!(identifier, "async");
    }

    #[test]
    fn keyword_into_identifier_context() {
        let identifier = Keyword::Yield
            .into_identifier_string(KeywordContext::empty())
            .unwrap();
        assert_eq!(identifier, "yield");

        let error = Keyword::Yield
            .into_identifier_string(KeywordContext::YIELD)
            .unwrap_err();
        assert_eq!(
            error,
            Error::of(ErrorKind::ForbiddenIdentifier(Keyword::Yield))
        );
    }

    #[test]
    fn reserved_word_into_identifier() {
        let error = Keyword::Function
            .into_identifier_string(KeywordContext::empty())
            .unwrap_err();
        assert_eq!(
            error,
            Error::of(ErrorKind::ForbiddenIdentifier(Keyword::Function))
        );
    }
}
