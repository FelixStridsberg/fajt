use crate::ast::Ident;
use fajt_lexer::token::Base as LexerBase;
use fajt_lexer::token::Literal as LexerLiteral;
use fajt_lexer::token::Number as LexerNumber;
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {
    This(ThisExpr),
    Literal(LiteralExpr),
    IdentifierReference(IdentifierReference),
}

impl From<ThisExpr> for Expr {
    fn from(expr: ThisExpr) -> Self {
        Self::This(expr)
    }
}

impl From<LiteralExpr> for Expr {
    fn from(expr: LiteralExpr) -> Self {
        Self::Literal(expr)
    }
}

impl From<IdentifierReference> for Expr {
    fn from(expr: IdentifierReference) -> Self {
        Self::IdentifierReference(expr)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct LiteralExpr {
    pub span: Span,
    pub literal: Literal,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Literal {
    Null,
    Boolean(bool),
    String(String, char),
    Number(Number),
    Array(Array),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Array {
    elements: Vec<Option<() /* TODO */>>,
}

impl Array {
    pub fn new(elements: Vec<Option<()>>) -> Self {
        Self { elements }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum Base {
    Binary,
    Decimal,
    Hex,
    Octal,
}

impl From<LexerBase> for Base {
    fn from(base: LexerBase) -> Self {
        match base {
            LexerBase::Binary => Base::Binary,
            LexerBase::Decimal => Base::Decimal,
            LexerBase::Hex => Base::Hex,
            LexerBase::Octal => Base::Octal,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Number {
    Integer(i64, Base),
    Decimal(f64),
}

impl From<LexerLiteral> for Literal {
    fn from(lexer_literal: LexerLiteral) -> Self {
        match lexer_literal {
            LexerLiteral::Number(LexerNumber::Integer(f, b)) => {
                Self::Number(Number::Integer(f, b.into()))
            }
            LexerLiteral::Number(LexerNumber::Decimal(f)) => Self::Number(Number::Decimal(f)),
            LexerLiteral::String(s, d) => Self::String(s, d),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ThisExpr {
    pub span: Span,
}

impl ThisExpr {
    pub fn new<S>(span: S) -> Self
    where
        S: Into<Span>,
    {
        Self { span: span.into() }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum IdentifierReference {
    Ident(Ident),
    Yield,
    Await,
}

impl From<Ident> for IdentifierReference {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}
