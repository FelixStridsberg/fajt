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

#[derive(Debug, PartialOrd, PartialEq, FromString)]
#[from_string_macro("punct")]
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
    #[from_string("%=")]
    PercentEqual,
    #[from_string("|=")]
    PipeEqual,
    #[from_string("^=")]
    CaretEqual,
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
pub enum TokenValue {
    Keyword(Keyword),
    Identifier(String),
    Number(Number),
    Assign(AssignOp),
    BitwiseShift(ShiftDirection),
    BinaryOperator(BinaryOp),
    UnaryOperation(UnaryOp),
    //Keyword(Keyword),
    //Ident(String),
    //Punct(Punctuator)
    //Literal(Literal),
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
