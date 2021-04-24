use fajt_lexer::token::Span;
use crate::ast::ProgramType::Script;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ProgramType {
    Script,
    Module,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    program_type: ProgramType,
    body: Vec<Stmt>,
}

impl Program {
    pub fn from_body(body: Vec<Stmt>) -> Self {
        Program {
            program_type: Script, // TODO check body
            body
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    Empty(EmptyStmt)
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