mod lib;

use fajt_lexer::punct;
use fajt_lexer::token;
use fajt_lexer::token::{Span, Token};
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::UnexpectedToken;

#[test]
fn empty_object_literal() {
    parser_test!(
        input: "{}",
        expr_output: [
            Expression::Literal(
                LiteralExpression {
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
fn single_prop_object_literal() {
    parser_test!(
        input: "{ a }",
        expr_output: [
            Expression::Literal(
                LiteralExpression {
                    span: Span::new(0, 5),
                    literal: Literal::Object(Object {
                        props: vec![
                            PropertyDefinition::IdentifierReference(Ident::new("a", (2, 3)).into())
                        ]
                    })
                }
            )
        ]
    );
}

#[test]
fn single_prop_object_literal_trailing_comma() {
    parser_test!(
        input: "{ a, }",
        expr_output: [
            Expression::Literal(
                LiteralExpression {
                    span: Span::new(0, 6),
                    literal: Literal::Object(Object {
                        props: vec![
                            PropertyDefinition::IdentifierReference(Ident::new("a", (2, 3)).into())
                        ]
                    })
                }
            )
        ]
    );
}

#[test]
fn multiple_props_object_literal() {
    parser_test!(
        input: "{ a,b, c }",
        expr_output: [
            Expression::Literal(
                LiteralExpression {
                    span: Span::new(0, 10),
                    literal: Literal::Object(Object {
                        props: vec![
                            PropertyDefinition::IdentifierReference(Ident::new("a", (2, 3)).into()),
                            PropertyDefinition::IdentifierReference(Ident::new("b", (4, 5)).into()),
                            PropertyDefinition::IdentifierReference(Ident::new("c", (7, 8)).into()),
                        ]
                    })
                }
            )
        ]
    );
}

#[test]
fn fail_object_literal_prefixing_comma() {
    parser_test!(
        input: "{ , a, b }",
        expr_error: UnexpectedToken(Token::new(punct!(","), (2, 3)))
    );
}

#[test]
fn fail_object_literal_double_comma() {
    parser_test!(
        input: "{ a,, b }",
        expr_error: UnexpectedToken(Token::new(punct!(","), (4, 5)))
    );
}

#[test]
fn object_literal_spread() {
    parser_test!(
        input: "{ ...a, ...b }",
        expr_output: [
            Expression::Literal(
                LiteralExpression {
                    span: Span::new(0, 14),
                    literal: Literal::Object(Object {
                        props: vec![
                            PropertyDefinition::Spread(
                                Expression::IdentifierReference(Ident::new("a", (5, 6)).into())
                            ),
                            PropertyDefinition::Spread(
                                Expression::IdentifierReference(Ident::new("b", (11, 12)).into())
                            )
                        ]
                    })
                }
            )
        ]
    );
}
