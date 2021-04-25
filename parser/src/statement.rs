use crate::ast::{
    BindingIdentifier, BindingPattern, Stmt, VariableDeclaration, VariableStmt, VariableType,
};
use crate::Parser;
use fajt_lexer::token::{Token, TokenValue};
use std::convert::TryInto;

impl Parser<'_> {
    pub(super) fn parse_variable_statement(&mut self, variable_type: VariableType) -> Stmt {
        let token = self.reader.next();
        if let TokenValue::Identifier(name) = &token.value {
            Stmt::VariableStmt(VariableStmt {
                variable_type,
                declarations: vec![self.parse_variable_declaration()],
            })
        } else {
            unimplemented!()
        }
    }

    fn parse_variable_declaration(&mut self) -> VariableDeclaration {
        let token = self.reader.current();
        VariableDeclaration {
            identifier: BindingPattern::Ident(BindingIdentifier::Ident(
                token.try_into().expect("Expected identifier"),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::VariableType::Var;
    use crate::ast::{
        BindingIdentifier, BindingPattern, EmptyStmt, Ident, Program, Stmt, VariableDeclaration,
        VariableStmt,
    };
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
                    variable_type: Var,
                    declarations: vec![
                        VariableDeclaration {
                            identifier: BindingPattern::Ident(BindingIdentifier::Ident(Ident {
                                span: (4, 7).into(),
                                name: "foo".to_string()
                            })),
                        }
                    ]
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
                    variable_type: Var,
                    declarations: vec![
                        VariableDeclaration {
                            identifier: BindingPattern::Ident(BindingIdentifier::Ident(Ident {
                                span: (4, 7).into(),
                                name: "foo".to_string()
                            })),
                        }
                    ]
                })
            ]
        );
    }
}
