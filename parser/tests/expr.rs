mod lib;

use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn this() {
    parser_test!(
        input: "this",
        output: [
            Expr::This(ThisExpr {
                span: Span::new(0, 4)
            }).into()
        ]
    );
}

#[test]
fn null_literal() {
    parser_test!(
        input: "null",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 4),
                    literal: Literal::Null,
                }
            ).into()
        ]
    );
}

#[test]
fn boolean_true_literal() {
    parser_test!(
        input: "true",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 4),
                    literal: Literal::Boolean(true),
                }
            ).into()
        ]
    );
}

#[test]
fn boolean_false_literal() {
    parser_test!(
        input: "false",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 5),
                    literal: Literal::Boolean(false),
                }
            ).into()
        ]
    );
}

#[test]
fn string_literal_double_quote() {
    parser_test!(
        input: r#""this is string""#,
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 16),
                    literal: Literal::String("this is string".to_owned(), '"'),
                }
            ).into()
        ]
    );
}

#[test]
fn string_literal_single_quote() {
    parser_test!(
        input: "'this is string'",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 16),
                    literal: Literal::String("this is string".to_owned(), '\''),
                }
            ).into()
        ]
    );
}

#[test]
fn decimal_integer_literal() {
    parser_test!(
        input: "1234",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 4),
                    literal: Literal::Number(
                        Number::Integer(1234, Base::Decimal)
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn decimal_hex_literal() {
    parser_test!(
        input: "0xff",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 4),
                    literal: Literal::Number(
                        Number::Integer(0xff, Base::Hex)
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn decimal_octal_literal() {
    parser_test!(
        input: "0o77",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 4),
                    literal: Literal::Number(
                        Number::Integer(0o77, Base::Octal)
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn decimal_binary_literal() {
    parser_test!(
        input: "0b11110000",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 10),
                    literal: Literal::Number(
                        Number::Integer(0b11110000, Base::Binary)
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn decimal_literal() {
    parser_test!(
        input: "1234.5",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 6),
                    literal: Literal::Number(
                        Number::Decimal(1234.5)
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn empty_array_literal() {
    parser_test!(
        input: "[]",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 2),
                    literal: Literal::Array(
                        Array::new(vec![])
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn elision_array_literal() {
    parser_test!(
        input: "[, ]",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 4),
                    literal: Literal::Array(
                        Array::new(vec![None])
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn single_element_array_literal() {
    parser_test!(
        input: "[ a ]",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 5),
                    literal: Literal::Array(
                        Array::new(vec![
                            Some(Expr::IdentifierReference(Ident::new("a", (2, 3)).into()))
                        ])
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn mixed_element_array_literal() {
    parser_test!(
        input: "[ a,, b ]",
        output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 9),
                    literal: Literal::Array(
                        Array::new(vec![
                            Some(Expr::IdentifierReference(Ident::new("a", (2, 3)).into())),
                            None,
                            Some(Expr::IdentifierReference(Ident::new("b", (6, 7)).into())),
                        ]),
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn empty_object_literal() {
    parser_test!(
        input: "{}",
        expr_output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 2),
                    literal: Literal::Object(Object {
                        props: vec![]
                    })
                }
            )
        ]
    );
}

#[test]
fn single_element_object_literal() {
    parser_test!(
        input: "{ a }",
        expr_output: [
            Expr::Literal(
                LiteralExpr {
                    span: Span::new(0, 5),
                    literal: Literal::Object(Object {
                        props: vec![
                            PropertyDefinition::IdentifierReference(
                                Ident::new("a", (2, 3)).into()
                            )
                        ]
                    })
                }
            )
        ]
    );
}
