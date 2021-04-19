use macros::FromString;

/// ECMAScript supports operators during variable assignment:
/// *=      Multiply
/// /=      Divide
/// %=      Modulus
/// +=      Add
/// -=      Subtract
/// <<=     LeftShift
/// >>=     RightShift
/// >>>=    URightShift
/// &=      BitwiseAnd
/// ^=      BitwiseXOr
/// |=      BitwiseOr
/// **=     Exponent
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum AssignOp {
    None,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, FromString)]
pub enum Keyword {
    Const,
    Let,
    Var,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Base {
    Decimal,
    Hex,
    Octal,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Number {
    Integer(i64, Base),
    Decimal(f64),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum TokenValue {
    Keyword(Keyword),
    Identifier(String),
    Number(Number),

    // let i = 1;
    //       ^
    Assign(AssignOp),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Token {
    pub value: TokenValue,
    pub location: Location,
}
