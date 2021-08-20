use crate::{Expr, Ident};

ast_node! {
    pub enum Literal {
        Null,
        Boolean(bool),
        String(String, char),
        Number(Number),
        Array(Array),
        Object(Object),
    }
}

impl Literal {
    pub fn unwrap_string(self) -> (String, char) {
        if let Literal::String(string, delimiter) = self {
            (string, delimiter)
        } else {
            panic!("Tried to unwrap {:?} as a string literal", self);
        }
    }
}

ast_node! {
    pub struct Array {
        pub elements: Vec<ArrayElement>,
    }
}

ast_node! {
    pub enum ArrayElement {
        None,
        Expr(Expr),
        Spread(Expr),
    }
}

ast_node! {
    pub struct Object {
        pub props: Vec<PropertyDefinition>,
    }
}

ast_node! {
    pub enum PropertyDefinition {
        IdentRef(Ident),
        Spread(Expr),
    }
}

ast_node! {
    pub enum Number {
        Integer(i64, Base),
        Decimal(f64),
    }
}

ast_node! {
    pub enum Base {
        Binary,
        Decimal,
        Hex,
        Octal,
    }
}
