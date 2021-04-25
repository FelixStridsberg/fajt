use crate::Parser;
use crate::ast::{Stmt, VariableDeclaration, VariableType};
use fajt_lexer::token::TokenValue;

impl Parser<'_> {
    pub(super) fn parse_variable_statement(&mut self) -> Stmt {
        let tok = self.reader.next();
        if let TokenValue::Identifier(name) = &tok.value {
            Stmt::VariableDeclaration(VariableDeclaration {
                variable_type: VariableType::Var,
                identifier: name.to_owned()
            })
        } else {
            unimplemented!()
        }
    }
}