mod lib;

use fajt_parser::ast::*;

#[test]
fn this() {
    parser_test!(
        input: "this",
        output: [Expr::This(ThisExpr::new((0, 4))).into()]
    );
}

#[test]
fn null_literal() {
    parser_test!(
        input: "null",
        output: [Expr::Literal(LiteralExpr::new(Literal::Null, (0, 4))).into()]
    );
}

#[test]
fn boolean_true_literal() {
    parser_test!(
        input: "true",
        output: [Expr::Literal(LiteralExpr::new(Literal::Boolean(true), (0, 4))).into()]
    );
}

#[test]
fn boolean_false_literal() {
    parser_test!(
        input: "false",
        output: [Expr::Literal(LiteralExpr::new(Literal::Boolean(false), (0, 5))).into()]
    );
}

#[test]
fn string_literal_double_quote() {
    parser_test!(
        input: r#""this is string""#,
        output: [
            Expr::Literal(LiteralExpr::new(
                Literal::String("this is string".to_owned(), '"'),
                (0, 16)
            )).into()
        ]
    );
}

#[test]
fn string_literal_single_quote() {
    parser_test!(
        input: "'this is string'",
        output: [
            Expr::Literal(LiteralExpr::new(
                Literal::String("this is string".to_owned(), '\''),
                (0, 16)
            )).into()
        ]
    );
}
