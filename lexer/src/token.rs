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
#[derive(Debug)]
pub enum AssignOp {
    None,
}

#[derive(Debug)]
pub enum Keyword {
    Const,
}

impl Keyword {
    pub fn from_string(string: &str) -> Option<Keyword> {
        match string {
            "const" => Some(Keyword::Const),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Base {
    Decimal,
    Hex,
    Octal,
}

#[derive(Debug)]
pub enum Number {
    Integer(i64, Base),
    Decimal(f64),
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Number(Number),

    // let i = 1;
    //       ^
    Assign(AssignOp),
}
