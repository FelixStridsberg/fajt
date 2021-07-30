use crate::ast::{Expr, Ident};

use fajt_lexer::token::Base as LexerBase;
use fajt_lexer::token::Literal as LexerLiteral;
use fajt_lexer::token::Number as LexerNumber;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Literal {
    Null,
    Boolean(bool),
    String(String, char),
    Number(Number),
    Array(Array),
    Object(Object),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Array {
    pub elements: Vec<ArrayElement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ArrayElement {
    None,
    Expr(Expr),
    Spread(Expr),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Object {
    pub props: Vec<PropertyDefinition>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum PropertyDefinition {
    IdentRef(Ident),
    Spread(Expr),
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
