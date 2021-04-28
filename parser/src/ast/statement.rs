use super::Ident;
use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    Empty(EmptyStmt),
    VariableStmt(VariableStmt),
}

impl From<EmptyStmt> for Stmt {
    fn from(stmt: EmptyStmt) -> Self {
        Self::Empty(stmt)
    }
}

impl From<VariableStmt> for Stmt {
    fn from(stmt: VariableStmt) -> Self {
        Self::VariableStmt(stmt)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct EmptyStmt {
    span: Span,
}

impl EmptyStmt {
    pub fn new(span: Span) -> Self {
        EmptyStmt { span }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableStmt {
    // TODO can have multiple declarations
    pub variable_type: VariableType,
    pub declarations: Vec<VariableDeclaration>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableDeclaration {
    pub identifier: BindingPattern,
    //pub initializer: TODO
}

impl VariableDeclaration {
    pub fn new<I: Into<BindingPattern>>(identifier: I) -> Self {
        VariableDeclaration {
            identifier: identifier.into(),
        }
    }
}

impl VariableStmt {
    pub fn new(variable_type: VariableType, declarations: Vec<VariableDeclaration>) -> Self {
        VariableStmt {
            variable_type,
            declarations,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum VariableType {
    Const,
    Let,
    Var,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingIdentifier {
    Ident(Ident),
    Yield,
    Await,
}

impl From<Ident> for BindingIdentifier {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

impl<I: Into<BindingIdentifier>> From<I> for ObjectBindingProp {
    fn from(ident: I) -> Self {
        Self::Assign(ident.into())
    }
}

impl From<Ident> for BindingPattern {
    fn from(ident: Ident) -> Self {
        Self::Ident(BindingIdentifier::Ident(ident))
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingPattern {
    Ident(BindingIdentifier),
    Object(ObjectBinding),
    Array(ArrayBinding),
}

impl<I: Into<ObjectBinding>> From<I> for BindingPattern {
    fn from(binding: I) -> Self {
        Self::Object(binding.into())
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ObjectBinding {
    pub props: Vec<ObjectBindingProp>,
}

impl From<Vec<ObjectBindingProp>> for ObjectBinding {
    fn from(props: Vec<ObjectBindingProp>) -> Self {
        Self { props }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ObjectBindingProp {
    Assign(BindingIdentifier), // TODO this can have Initializer as well
                               // KeyValue
                               // Rest
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ArrayBinding {
    pub elements: Vec<Option<BindingPattern>>,
}
