use crate::{Expr, Ident, MethodDefinition, PropertyName, Span};

ast_node! {
    pub enum Literal {
        Null,
        Boolean(bool),
        String(LitString),
        Number(LitNumber),
        Array(LitArray),
        Object(LitObject),
        Regexp(String),
        Template(LitTemplate),
    }
}

ast_node! {
    pub enum TemplatePart {
        String(String),
        Expr(Box<Expr>),
    }
}

impl Literal {
    pub fn unwrap_string(self) -> LitString {
        if let Literal::String(string) = self {
            string
        } else {
            panic!("Tried to unwrap {self:?} as a string literal");
        }
    }
}

ast_node! {
    pub struct LitString {
        pub value: String,
        pub delimiter: char,
    }
}

ast_node! {
    pub struct LitTemplate {
        pub parts: Vec<TemplatePart>,
    }
}

ast_node! {
    pub struct LitArray {
        pub elements: Vec<ArrayElement>,
    }
}

ast_node! {
    pub enum ArrayElement {
        Elision,
        Expr(Expr),
        Spread(Expr),
    }
}

ast_node! {
    pub struct LitObject {
        pub props: Vec<PropertyDefinition>,
    }
}

ast_node! {
    pub enum PropertyDefinition {
        IdentRef(Ident),
        Spread(Expr),
        Named(NamedProperty),
        Method(MethodDefinition),
    }
}

ast_struct! {
    pub struct NamedProperty {
        pub span: Span,
        pub name: PropertyName,
        pub value: Expr,
    }
}

ast_node! {
    pub enum LitNumber {
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
