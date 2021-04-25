use crate::ast::{BindingPattern, Stmt, VariableStmt, VariableType};
use crate::Parser;
use fajt_lexer::token::TokenValue;
use std::convert::TryInto;

impl Parser<'_> {
    pub(super) fn parse_variable_statement(&mut self) -> Stmt {
        let tok = self.reader.next();
        if let TokenValue::Identifier(name) = &tok.value {
            Stmt::VariableStmt(VariableStmt {
                variable_type: VariableType::Var,
                identifier: BindingPattern::Ident(tok.try_into().expect("Expected identifier")),
            })
        } else {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::VariableType::Var;
    use crate::ast::{BindingPattern, EmptyStmt, Ident, Program, Stmt, VariableStmt};
    use crate::{Parser, Reader};
    use fajt_lexer::Lexer;

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
            input: "var foo = 1;",
            output: [
                Stmt::VariableStmt(VariableStmt {
                    identifier: BindingPattern::Ident(Ident {
                        span: (4, 7).into(),
                        name: "foo".to_string()
                    }),
                    variable_type: Var,
                })
            ]
        );
    }

    #[test]
    fn parse_var_stmt_empty_obj_binding() {
        parser_test!(
            input: "var {} = 1;",
            output: [
                Stmt::VariableStmt(VariableStmt {
                    identifier: BindingPattern::Ident(Ident {
                        span: (4, 7).into(),
                        name: "foo".to_string()
                    }),
                    variable_type: Var,
                })
            ]
        );
    }
}
