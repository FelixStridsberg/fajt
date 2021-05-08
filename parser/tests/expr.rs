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

#[test]
fn decimal_literal() {
    parser_test!(
        input: "1234",
        output: [
            Expr::Literal(LiteralExpr::new(
                Literal::Number(
                    Number::Integer(1234, Base::Decimal)
                ),
                (0, 4)
            )).into()
        ]
    );
}

#[test]
fn decimal_hex_literal() {
    parser_test!(
        input: "0xff",
        output: [
            Expr::Literal(LiteralExpr::new(
                Literal::Number(
                    Number::Integer(0xff, Base::Hex)
                ),
                (0, 4)
            )).into()
        ]
    );
}

#[test]
fn decimal_octal_literal() {
    parser_test!(
        input: "0o77",
        output: [
            Expr::Literal(LiteralExpr::new(
                Literal::Number(
                    Number::Integer(0o77, Base::Octal)
                ),
                (0, 4)
            )).into()
        ]
    );
}

#[test]
fn decimal_binary_literal() {
    parser_test!(
        input: "0b11110000",
        output: [
            Expr::Literal(LiteralExpr::new(
                Literal::Number(
                    Number::Integer(0b11110000, Base::Binary)
                ),
                (0, 10)
            )).into()
        ]
    );
}