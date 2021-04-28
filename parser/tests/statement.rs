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
fn parse_var_statement() {
    parser_test!(
        input: "var foo = 1;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(Ident::new("foo", (4, 7)))
            ], (0, 7 /* TODO wrong */)).into()
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
            ], (0, 6 /* TODO wrong */)).into()
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
            ], (0, 9 /* TODO wrong */)).into()
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
            ], (0, 12 /* TODO wrong */)).into()
        ]
    );
}
