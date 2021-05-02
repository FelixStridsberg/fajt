use crate::ast::{
    BindingIdentifier, BindingPattern, ObjectBindingProp, Stmt, VariableDeclaration, VariableKind,
    VariableStmt,
};
use crate::Parser;
use fajt_lexer::punct;
use fajt_lexer::token::TokenValue;
use fajt_lexer::token_matches;
use std::convert::TryInto;

impl Parser<'_> {
    pub(crate) fn parse_variable_statement(&mut self, variable_type: VariableKind) -> Stmt {
        let start = self.reader.current().location.start;

        // TODO parse all declarations
        let declarations = vec![self.parse_variable_declaration()];
        let end = self.reader.current().location.end;

        VariableStmt::new(variable_type, declarations, (start, end)).into()
    }

    fn parse_variable_declaration(&mut self) -> VariableDeclaration {
        println!("TOKEN: {:?}", self.reader.current());
        let token = self.reader.next().unwrap();
        println!("TOKEN: {:?}", token);

        let identifier = match &token.value {
            TokenValue::Identifier(_) => BindingPattern::Ident(BindingIdentifier::Ident(
                token.try_into().expect("Expected identifier"),
            )),
            punct!("{") => self.parse_object_property_binding(),
            punct!("[") => unimplemented!("Array binding"),
            c => unimplemented!("{:?}", c),
        };

        match self.reader.peek() {
            token_matches!(punct!("=")) => self.parse_variable_initializer(),
            token_matches!(punct!(";")) => {
                self.reader.next().unwrap(); // TODO fix unwrap
            }
            _ => (),
        }

        VariableDeclaration { identifier }
    }

    fn parse_variable_initializer(&mut self) {
        // TODO read initializer
        loop {
            self.reader.next();
            if punct!(";") == self.reader.current().value {
                break;
            }
        }
    }

    fn parse_object_property_binding(&mut self) -> BindingPattern {
        let mut bindings = Vec::new();

        loop {
            let token = self.reader.next().unwrap(); // TODO

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
