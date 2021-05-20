use fajt_lexer::token::{Span, Token, TokenValue};
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::UnexpectedToken;

#[test]
fn empty_array_literal() {
    parser_test!(
        input: "[]",
        output: [
            Expression::Literal(
                LiteralExpression {
                    span: Span::new(0, 2),
                    literal: Literal::Array(
                        Array { elements: vec![] }
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn fail_array_literal_missing_comma() {
    parser_test!(
        input: "[a b]",
        error: UnexpectedToken(Token::new(TokenValue::Identifier("b".to_owned()), false, (3, 4)))
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
                        Array { elements: vec![ArrayElement::None] }
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
                        Array {
                            elements: vec![
                                ArrayElement::Expr(
                                    Expression::IdentifierReference(Ident::new("a", (2, 3)).into())
                                )
                            ]
                        }
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
                        Array {
                            elements: vec![
                                ArrayElement::Expr(
                                    Expression::IdentifierReference(Ident::new("a", (2, 3)).into())
                                ),
                                ArrayElement::None,
                                ArrayElement::Expr(
                                    Expression::IdentifierReference(Ident::new("b", (6, 7)).into())
                                ),
                            ]
                        },
                    ),
                }
            ).into()
        ]
    );
}

#[test]
fn array_literal_spread_element() {
    parser_test!(
        input: "[ ...a ]",
        output: [
            Expression::Literal(
                LiteralExpression {
                    span: Span::new(0, 8),
                    literal: Literal::Array(
                        Array {
                            elements: vec![
                                ArrayElement::Spread(
                                    Expression::IdentifierReference(Ident::new("a", (5, 6)).into())
                                )
                            ]
                        },
                    ),
                }
            ).into()
        ]
    );
}
