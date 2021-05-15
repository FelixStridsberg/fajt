use super::Ident;
use crate::ast::Expression;
use fajt_lexer::token::Span;

/// Note: Declarations are handles as statements since they can appear in the same contexts.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Statement {
    Block(BlockStatement),
    Empty(EmptyStatement),
    Variable(VariableStatement),
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
}

impl From<BlockStatement> for Statement {
    fn from(stmt: BlockStatement) -> Self {
        Self::Block(stmt)
    }
}

impl From<EmptyStatement> for Statement {
    fn from(stmt: EmptyStatement) -> Self {
        Self::Empty(stmt)
    }
}

impl From<VariableStatement> for Statement {
    fn from(stmt: VariableStatement) -> Self {
        Self::Variable(stmt)
    }
}

impl From<Expression> for Statement {
    fn from(expr: Expression) -> Self {
        Self::Expression(expr)
    }
}

impl From<FunctionDeclaration> for Statement {
    fn from(expr: FunctionDeclaration) -> Self {
        Self::FunctionDeclaration(expr)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct FunctionDeclaration {
    pub span: Span,
    pub ident: Ident,
    pub parameters: Vec<()>, // TODO
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct BlockStatement {
    pub span: Span,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct EmptyStatement {
    pub span: Span,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableStatement {
    pub span: Span,
    pub kind: VariableKind,
    pub declarations: Vec<VariableDeclaration>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct VariableDeclaration {
    pub span: Span,
    pub identifier: BindingPattern,
    pub initializer: Option<Expression>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum VariableKind {
    Const,
    Let,
    Var,
}

impl From<Ident> for BindingPattern {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BindingPattern {
    Ident(Ident),
    Object(ObjectBinding),
    Array(ArrayBinding),
}

impl From<ObjectBinding> for BindingPattern {
    fn from(binding: ObjectBinding) -> Self {
        Self::Object(binding)
    }
}

impl From<ArrayBinding> for BindingPattern {
    fn from(binding: ArrayBinding) -> Self {
        Self::Array(binding)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ObjectBinding {
    pub span: Span,
    pub props: Vec<ObjectBindingProp>,
    pub rest: Option<Ident>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ObjectBindingProp {
    Assign(Ident), // TODO this can have Initializer as well
                   // KeyValue
}

impl From<Ident> for ObjectBindingProp {
    fn from(ident: Ident) -> Self {
        Self::Assign(ident)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ArrayBinding {
    pub span: Span,
    pub elements: Vec<Option<BindingPattern>>,
    pub rest: Option<Ident>,
}
