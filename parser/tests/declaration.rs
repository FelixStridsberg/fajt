mod r#macro;

use fajt_lexer::token::Keyword;
use fajt_lexer::token::TokenValue;
use fajt_lexer::token::{Span, Token};
use fajt_parser::ast::Base::Decimal;
use fajt_parser::ast::Expr::IdentifierReference;
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::UnexpectedToken;

#[test]
fn function_declaration() {
    parser_test!(
        input: "function fn() {}",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 16),
                asynchronous: false,
                generator: false,
                identifier: Ident::new("fn", (9, 11)),
                parameters: FormalParameters::empty((11, 13)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn generator_declaration() {
    parser_test!(
        input: "function *fn() {}",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 17),
                asynchronous: false,
                generator: true,
                identifier: Ident::new("fn", (10, 12)),
                parameters: FormalParameters::empty((12, 14)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn function_declaration_with_body() {
    parser_test!(
        input: "function fn() { var a = 1 }",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 27),
                asynchronous: false,
                generator: false,
                identifier: Ident::new("fn", (9, 11)),
                parameters: FormalParameters::empty((11, 13)),
                body: vec![
                    VariableStatement {
                        span: Span::new(16, 25),
                        kind: VariableKind::Var,
                        declarations: vec![
                            VariableDeclaration {
                                span: Span::new(20, 25),
                                pattern: Ident::new("a", (20, 21)).into(),
                                initializer: Some(
                                    LiteralExpression {
                                        span: Span::new(24, 25),
                                        literal: Literal::Number(Number::Integer(1, Decimal))
                                    }.into()
                                )
                            }
                        ]
                    }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn async_function_declaration() {
    parser_test!(
        input: "async function fn() {}",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 22),
                asynchronous: true,
                generator: false,
                identifier: Ident::new("fn", (15, 17)),
                parameters: FormalParameters::empty((17, 19)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn async_generator_declaration() {
    parser_test!(
        input: "async function *fn() {}",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 23),
                asynchronous: true,
                generator: true,
                identifier: Ident::new("fn", (16, 18)),
                parameters: FormalParameters::empty((18, 20)),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn async_function_declaration_with_body() {
    parser_test!(
        input: "async function fn() { var a = 1 }",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 33),
                asynchronous: true,
                generator: false,
                identifier: Ident::new("fn", (15, 17)),
                parameters: FormalParameters::empty((17, 19)),
                body: vec![
                    VariableStatement {
                        span: Span::new(22, 31),
                        kind: VariableKind::Var,
                        declarations: vec![
                            VariableDeclaration {
                                span: Span::new(26, 31),
                                pattern: Ident::new("a", (26, 27)).into(),
                                initializer: Some(
                                    LiteralExpression {
                                        span: Span::new(30, 31),
                                        literal: Literal::Number(Number::Integer(1, Decimal))
                                    }.into()
                                )
                            }
                        ]
                    }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn await_keyword_inside_function() {
    parser_test!(
        input: "function fn() { var await = 1 }",
        success
    );
}

#[test]
fn fail_await_keyword_inside_async_function() {
    parser_test!(
        input: "async function fn() { var await = 1 }",
        error: UnexpectedToken(Token::new(TokenValue::Keyword(Keyword::Await), false, (26, 31)))
    );
}

#[test]
fn function_declaration_with_parameters() {
    parser_test!(
        input: "function fn(a, b = c) { }",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 25),
                asynchronous: false,
                generator: false,
                identifier: Ident::new("fn", (9, 11)),
                parameters: FormalParameters {
                    span: Span::new(11, 21),
                    bindings: vec![
                        BindingElement {
                            span: Span::new(12, 13),
                            pattern: BindingPattern::Ident(Ident::new("a", (12, 13))),
                            initializer: None,
                        },
                        BindingElement {
                            span: Span::new(15, 20),
                            pattern: BindingPattern::Ident(Ident::new("b", (15, 16))),
                            initializer: Some(IdentifierReference(
                                Box::new(Ident::new("c", (19, 20)).into())
                            )),
                        }
                    ],
                    rest: None
                },
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn function_declaration_with_rest_parameter() {
    parser_test!(
        input: "function fn(...a) { }",
        output: [
            FunctionDeclaration {
                span: Span::new(0, 21),
                asynchronous: false,
                generator: false,
                identifier: Ident::new("fn", (9, 11)),
                parameters: FormalParameters {
                    span: Span::new(11, 17),
                    bindings: vec![],
                    rest: Some(BindingPattern::Ident(Ident::new("a", (15, 16))))
                },
                body: vec![],
            }.into()
        ]
    );
}
