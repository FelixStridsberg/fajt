mod lib;

use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_array_literal() {
    parser_test!(
        input: "[]",
        output: [
            Expression::Literal(
                LiteralExpression {
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
            Expression::Literal(
                LiteralExpression {
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
            Expression::Literal(
                LiteralExpression {
                    span: Span::new(0, 5),
                    literal: Literal::Array(
                        Array::new(vec![
                            Some(Expression::IdentifierReference(Ident::new("a", (2, 3)).into()))
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
            Expression::Literal(
                LiteralExpression {
                    span: Span::new(0, 9),
                    literal: Literal::Array(
                        Array::new(vec![
                            Some(Expression::IdentifierReference(Ident::new("a", (2, 3)).into())),
                            None,
                            Some(Expression::IdentifierReference(Ident::new("b", (6, 7)).into())),
                        ]),
                    ),
                }
            ).into()
        ]
    );
}
