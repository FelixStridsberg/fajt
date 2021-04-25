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

#[cfg(test)]
mod tests {
    use crate::{Parser, Reader};
    use fajt_lexer::Lexer;
    use crate::ast::{Program, Stmt, EmptyStmt, VariableDeclaration};
    use crate::ast::VariableType::Var;

    #[test]
    fn parse_empty_statement() {
        parser_test!(
            input: ";",
            output: [Stmt::Empty(EmptyStmt::new((0, 1).into()))]
        );
    }

    #[test]
    fn parse_var_statement() {
        parser_test!(
            input: ";",
            output: [
                Stmt::VariableDeclaration(VariableDeclaration {
                    identifier: "foo".to_owned(),
                    variable_type: Var,
                })
            ]
        );
    }
}
