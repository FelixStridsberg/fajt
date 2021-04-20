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
/// TODO revisit naming of these
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum AssignOp {
    Multiply,
    Divide,
    Modulus,
    Add,
    Subtract,
    LeftShift,
    RightShift,
    URightShift,
    BitwiseAnd,
    BitwiseXOr,
    BitwiseOr,
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
pub enum ShiftDirection {
    Left,
    Right,
    UnsignedRight,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum TokenValue {
    Keyword(Keyword),
    Identifier(String),
    Number(Number),
    Assign(AssignOp),
    BitwiseShift(ShiftDirection),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Into<Position> for (u32, u32) {
    fn into(self) -> Position {
        Position {
            line: self.0,
            column: self.1,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

impl<P: Into<Position>> Into<Location> for (P, P) {
    fn into(self) -> Location {
        Location {
            start: self.0.into(),
            end: self.1.into(),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Token {
    pub value: TokenValue,
    pub location: Location,
}

impl Token {
    pub fn new<L: Into<Location>>(value: TokenValue, location: L) -> Self {
        Token {
            value,
            location: location.into(),
        }
    }
}
