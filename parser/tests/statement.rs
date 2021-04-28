mod lib;

use fajt_parser::ast::VariableType::*;
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
            ]).into()
        ]
    );
}

#[test]
fn parse_var_stmt_empty_obj_binding() {
    parser_test!(
        input: "var {} = 1;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(BindingPattern::Object(ObjectBinding { props: vec![] })),
            ]).into()
        ]
    );
}

#[test]
fn parse_var_stmt_empty_single_binding() {
    parser_test!(
        input: "var { a } = b;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(ObjectBinding {
                    props: vec![
                        Ident::new("a", (6, 7)).into()
                    ]
                }),
            ]).into()
        ]
    );
}

#[test]
fn parse_var_stmt_empty_multiple_binding() {
    parser_test!(
        input: "var { a, b } = c;",
        output: [
            VariableStmt::new(Var, vec![
                    VariableDeclaration::new(ObjectBinding {
                        props: vec![
                            Ident::new("a", (6, 7)).into(),
                            Ident::new("b", (9, 10)).into(),
                        ]
                    }),
                ]
            ).into()
        ]
    );
}
