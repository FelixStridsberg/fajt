mod lib;

use fajt_lexer::punct;
use fajt_lexer::token;
use fajt_lexer::token::Token;
use fajt_parser::ast::VariableKind::*;
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::{SyntaxError, UnexpectedToken};

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
                VariableDeclaration::new(Ident::new("foo", (4, 7)), None, (4, 8))
            ], (0, 8)).into()
        ]
    );
}

#[test]
fn parse_var_statement() {
    parser_test!(
        input: "var foo = a;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(Ident::new("foo", (4, 7)), None, (4, 12))
            ], (0, 12)).into()
        ]
    );
}

#[test]
fn fail_var_invalid_identifier() {
    parser_test!(
        input: "var * = c;",
        error: UnexpectedToken(Token::new(punct!("*"), (4, 5)))
    );
}

#[test]
fn parse_var_stmt_empty_obj_binding() {
    parser_test!(
        input: "var {} = a;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(
                    BindingPattern::Object(
                        ObjectBinding::new(vec![], None, (4, 6))
                    ), None, (4, 11)),
            ], (0, 11)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_obj_binding() {
    parser_test!(
        input: "var { a } = b;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(ObjectBinding::new(
                    vec![Ident::new("a", (6, 7)).into()],
                    None,
                    (4, 9)
                ), None, (4, 14)),
            ], (0, 14)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_multiple_obj_binding() {
    parser_test!(
        input: "var { a, b } = c;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(ObjectBinding::new(
                    vec![
                        Ident::new("a", (6, 7)).into(),
                        Ident::new("b", (9, 10)).into(),
                    ],
                    None,
                    (4, 12)
                ), None, (4, 17))
            ], (0, 17)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_obj_binding_rest() {
    parser_test!(
        input: "var { ...rest } = c;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(ObjectBinding::new(
                    vec![],
                    Some(Ident::new("rest", (9, 13))),
                    (4, 15)
                ), None, (4, 20))
            ], (0, 20)).into()
        ]
    );
}

/// `await` is a valid identifier in the parser context.
/// See the goal symbol `BindingIdentifier` specification.
#[test]
fn parse_var_stmt_obj_binding_await() {
    parser_test!(
        input: "var { await, ...await } = c;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(ObjectBinding::new(
                    vec![
                        Ident::new("await", (6, 11)).into(),
                    ],
                    Some(Ident::new("await", (16, 21))),
                    (4, 23)
                ), None, (4, 28))
            ], (0, 28)).into()
        ]
    );
}

/// `yield` is a valid identifier in the parser context.
/// See the goal symbol `BindingIdentifier` specification.
#[test]
fn parse_var_stmt_obj_binding_yield() {
    parser_test!(
        input: "var { yield, ...yield } = c;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(ObjectBinding::new(
                    vec![
                        Ident::new("yield", (6, 11)).into(),
                    ],
                    Some(Ident::new("yield", (16, 21))),
                    (4, 23)
                ), None, (4, 28))
            ], (0, 28)).into()
        ]
    );
}

#[test]
fn fail_var_statement_prefix_comma() {
    parser_test!(
        input: "var { , a, b } = c;",
        error: UnexpectedToken(Token::new(punct!(","), (6, 7)))
    );
}

#[test]
fn fail_var_statement_double_comma() {
    parser_test!(
        input: "var { a,, b } = c;",
        error: UnexpectedToken(Token::new(punct!(","), (8, 9)))
    );
}

#[test]
fn parse_var_stmt_empty_array_binding() {
    parser_test!(
        input: "var [] = a;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(
                    BindingPattern::Array(
                        ArrayBinding::new(vec![], None, (4, 6))
                    ), None, (4, 11)),
            ], (0, 11)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_elem_array_binding() {
    parser_test!(
        input: "var [ a ] = b;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(
                    BindingPattern::Array(
                        ArrayBinding::new(vec![
                            Some(BindingPattern::Ident(Ident::new("a", (6, 7)).into()))
                        ], None, (4, 9))
                    ), None, (4, 14)),
            ], (0, 14)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_elem_array_binding_trailing_comma() {
    parser_test!(
        input: "var [ a, ] = b;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(
                    BindingPattern::Array(
                        ArrayBinding::new(vec![
                            Some(BindingPattern::Ident(Ident::new("a", (6, 7)).into()))
                        ], None, (4, 10))
                    ), None, (4, 15)),
            ], (0, 15)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_elision_array_binding() {
    parser_test!(
        input: "var [ , ] = b;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(
                    BindingPattern::Array(
                        ArrayBinding::new(vec![
                            None
                        ], None, (4, 9))
                    ), None, (4, 14)),
            ], (0, 14)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_single_rest_array_binding() {
    parser_test!(
        input: "var [ ...a ] = b;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(
                    BindingPattern::Array(
                        ArrayBinding::new(vec![
                        ], Some(Ident::new("a", (9, 10))), (4, 12))
                    ), None, (4, 17)),
            ], (0, 17)).into()
        ]
    );
}

#[test]
fn parse_var_stmt_array_binding_mixed() {
    parser_test!(
        input: "var [ , a,,b ] = c;",
        output: [
            VariableStmt::new(Var, vec![
                VariableDeclaration::new(
                    BindingPattern::Array(
                        ArrayBinding::new(vec![
                            None,
                            Some(BindingPattern::Ident(Ident::new("a", (8, 9)).into())),
                            None,
                            Some(BindingPattern::Ident(Ident::new("b", (11, 12)).into())),
                        ], None, (4, 14))
                    ), None, (4, 19)),
            ], (0, 19)).into()
        ]
    );
}

#[test]
fn fail_var_stmt_array_biding_rest_not_last() {
    parser_test!(
        input: "var [ ...rest, b ] = c;",
        error: SyntaxError("Rest element must be last element".to_owned(), (9, 13).into())
    );
}
