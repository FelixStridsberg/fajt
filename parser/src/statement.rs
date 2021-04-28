use crate::ast::{
    BindingIdentifier, BindingPattern, ObjectBindingProp, Stmt, VariableDeclaration, VariableStmt,
    VariableType,
};
use crate::Parser;
use fajt_lexer::punct;
use fajt_lexer::token::TokenValue;
use std::convert::TryInto;

impl Parser<'_> {
    pub(super) fn parse_variable_statement(&mut self, variable_type: VariableType) -> Stmt {
        Stmt::VariableStmt(VariableStmt {
            variable_type,
            declarations: vec![self.parse_variable_declaration()],
        })
    }

    fn parse_variable_declaration(&mut self) -> VariableDeclaration {
        let token = self.reader.next();

        let identifier = match &token.value {
            TokenValue::Identifier(_) => BindingPattern::Ident(BindingIdentifier::Ident(
                token.try_into().expect("Expected identifier"),
            )),
            punct!("{") => self.parse_object_property_binding(),
            punct!("[") => unimplemented!("Array binding"),
            _ => unimplemented!(),
        };

        VariableDeclaration { identifier }
    }

    fn parse_object_property_binding(&mut self) -> BindingPattern {
        let mut bindings = Vec::new();

        loop {
            let token = self.reader.next();

            match token.value {
                punct!("}") => break,
                punct!(",") => {} // TODO verify correct placement of comma
                TokenValue::Identifier(_) => bindings.push(ObjectBindingProp::Assign(
                    BindingIdentifier::Ident(token.try_into().unwrap()),
                )),
                _ => unimplemented!(),
            }
        }

        bindings.into()
    }
}
