use crate::ast::{
    BindingIdentifier, BindingPattern, Expr, ObjectBindingProp, Stmt, VariableDeclaration,
    VariableKind, VariableStmt,
};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::{Error, Result};
use crate::Parser;
use fajt_lexer::punct;
use fajt_lexer::token::TokenValue;
use fajt_lexer::token_matches;
use std::convert::TryInto;

impl Parser<'_> {
    pub(crate) fn parse_variable_statement(&mut self, variable_type: VariableKind) -> Result<Stmt> {
        let start = self.reader.current().location.start;

        // TODO parse all declarations
        let declarations = vec![self.parse_variable_declaration()?];
        let end = self.reader.current().location.end;

        Ok(VariableStmt::new(variable_type, declarations, (start, end)).into())
    }

    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let token = self.reader.next().unwrap();

        let identifier = match &token.value {
            TokenValue::Identifier(_) => BindingPattern::Ident(BindingIdentifier::Ident(
                token.try_into().expect("Expected identifier"),
            )),
            punct!("{") => self.parse_object_property_binding()?,
            punct!("[") => unimplemented!("Array binding"),
            c => unimplemented!("{:?}", c),
        };

        let mut initializer = None;
        match self.reader.peek() {
            token_matches!(opt: punct!("=")) => {
                initializer = Some(self.parse_variable_initializer())
            }
            token_matches!(opt: punct!(";")) => {
                self.reader.next().unwrap(); // TODO fix unwrap
            }
            _ => (),
        }

        Ok(VariableDeclaration::new(identifier))
    }

    fn parse_variable_initializer(&mut self) -> Result<Expr> {
        self.reader.next()?; // Skip =
        self.reader.next()?; // Skip =
        self.parse_assignment_expression()
    }

    fn parse_object_property_binding(&mut self) -> Result<BindingPattern> {
        let mut bindings = Vec::new();

        let mut comma_allowed = false;
        loop {
            let token = self.reader.next().unwrap(); // TODO

            match token {
                token_matches!(punct!("}")) => break,
                token_matches!(punct!(",")) if comma_allowed => comma_allowed = false,
                token_matches!(@ident) => {
                    comma_allowed = true;
                    bindings.push(ObjectBindingProp::Assign(BindingIdentifier::Ident(
                        token.try_into().unwrap(),
                    )))
                }
                t => return Err(Error::of(UnexpectedToken(t.clone()))),
            }
        }

        Ok(bindings.into())
    }
}
