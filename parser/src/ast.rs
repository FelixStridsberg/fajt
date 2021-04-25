use fajt_lexer::token::Span;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ModuleItem {}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Body<T> {
    span: Span,
    body: Vec<T>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Program {
    Script(Body<Stmt>),
    Module(Body<ModuleItem>),
}

impl Program {
    pub fn from_body(body: Vec<Stmt>) -> Self {
        Program::Script(Body {
            span: (0, 0).into(),
            body,
        })
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum VariableType {
    Const,
    Let,
    Var,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableDeclaration {
    // TODO can have multiple declarations
    pub variable_type: VariableType,
    pub identifier: String, // TODO pattern
    //pub initializer: TODO
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    Empty(EmptyStmt),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct EmptyStmt {
    span: Span
}

impl EmptyStmt {
    pub fn new(span: Span) -> Self {
        EmptyStmt { span }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {}
