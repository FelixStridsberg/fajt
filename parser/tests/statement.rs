mod lib;

use fajt_parser::ast::VariableKind::*;
use fajt_parser::ast::*;

#[test]
fn parse_empty_statement() {
    parser_test!(
        input: ";",
        output: [EmptyStmt::new((0, 1).into()).into()]
    );
}

#[test]
fn parse_var_statement_no_init() {
    parser_test!(
        input: "var foo;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(Ident::new("foo", (4, 7)))
            ], (0, 8)).into()
        ]
    );
}

#[test]
fn parse_var_statement() {
    parser_test!(
        input: "var foo = 1;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(Ident::new("foo", (4, 7)))
            ], (0, 12)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_empty_obj_binding() {
    parser_test!(
        input: "var {} = 1;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(BindingPattern::Object(ObjectBinding::new(vec![]))),
            ], (0, 11)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_empty_single_binding() {
    parser_test!(
        input: "var { a } = b;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(ObjectBinding::new(vec![
                    Ident::new("a", (6, 7)).into()
                ])),
            ], (0, 14)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_empty_multiple_binding() {
    parser_test!(
        input: "var { a, b } = c;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(ObjectBinding::new(vec![
                    Ident::new("a", (6, 7)).into(),
                    Ident::new("b", (9, 10)).into(),
                ]))
            ], (0, 17)).into()
        ]
    );
}

#[test]
fn fail_var_statement_prefix_comma() {
    parser_test!(
        input: "var { , a, b } = c;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(ObjectBinding::new(vec![
                    Ident::new("a", (6, 7)).into(),
                    Ident::new("b", (9, 10)).into(),
                ]))
            ], (0, 17)).into()
        ]
    );
}