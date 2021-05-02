use crate::ast::Ident;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {
    IdentifierReference(IdentifierReference),
}

impl From<IdentifierReference> for Expr {
    fn from(expr: IdentifierReference) -> Self {
        Self::IdentifierReference(expr)
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
