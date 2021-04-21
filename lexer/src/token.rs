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
    Exponent,
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

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ShiftDirection {
    Left,
    Right,
    UnsignedRight,
}

#[derive(Debug, PartialOrd, PartialEq)]
// TODO, don't like this BinaryOperator -> BinaryOp
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Exponent,
    Divide,
    Modulus,
    LeftShift,
    RightShift,
    URightShift,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXOr,
    LogicalOr,
    LogicalAnd,
    Coalesce,
}

#[derive(Debug, PartialOrd, PartialEq)]
// TODO, don't like this UnaryOperator -> UnaryOp
pub enum UnaryOp {
    Positive,
    Negative,
    BitwiseNot,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum TokenValue {
    Keyword(Keyword),
    Identifier(String),
    Number(Number),
    Assign(AssignOp),
    BitwiseShift(ShiftDirection),
    BinaryOperator(BinaryOp),
    UnaryOperation(UnaryOp),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct FilePosition {
    pub line: usize,
    pub column: usize,
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
