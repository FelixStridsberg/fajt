mod lib;

use fajt_parser::ast::VariableType::*;
use fajt_parser::ast::*;

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
            Stmt::VariableStmt(
                VariableStmt::new(Var, vec![
                    VariableDeclaration::new(Ident::new("foo", (4, 7)))
                ])
            )
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
                    VariableDeclaration::new(BindingPattern::Object(ObjectBinding { props: vec![] })),
                ]
            })
        ]
    );
}

#[test]
fn parse_var_stmt_empty_single_binding() {
    parser_test!(
        input: "var { a } = b;",
        output: [
            Stmt::VariableStmt(VariableStmt {
                variable_type: Var,
                declarations: vec![
                    VariableDeclaration::new(BindingPattern::Object(ObjectBinding {
                        props: vec![
                            Ident::new("a", (6, 7)).into()
                        ]
                    })),
                ]
            })
        ]
    );
}

#[test]
fn parse_var_stmt_empty_multiple_binding() {
    parser_test!(
        input: "var { a, b } = c;",
        output: [
            Stmt::VariableStmt(VariableStmt::new(Var, vec![
                    VariableDeclaration {
                        identifier: BindingPattern::Object(ObjectBinding {
                            props: vec![
                            Ident::new("a", (6, 7)).into(),
                            Ident::new("b", (9, 10)).into(),
                            ]
                        }),
                    }
                ]
            ))
        ]
    );
}
