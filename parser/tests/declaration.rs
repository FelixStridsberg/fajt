mod lib;

use fajt_lexer::token::Keyword;
use fajt_lexer::token::TokenValue;
use fajt_lexer::token::{Span, Token};
use fajt_parser::ast::Base::Decimal;
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
                ident: Ident::new("fn", (9, 11)),
                parameters: vec![],
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
                ident: Ident::new("fn", (9, 11)),
                parameters: vec![],
                body: vec![
                    VariableStatement {
                        span: Span::new(16, 25),
                        kind: VariableKind::Var,
                        declarations: vec![
                            VariableDeclaration {
                                span: Span::new(20, 25),
                                identifier: Ident::new("a", (20, 21)).into(),
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
                ident: Ident::new("fn", (15, 17)),
                parameters: vec![],
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
                ident: Ident::new("fn", (15, 17)),
                parameters: vec![],
                body: vec![
                    VariableStatement {
                        span: Span::new(22, 31),
                        kind: VariableKind::Var,
                        declarations: vec![
                            VariableDeclaration {
                                span: Span::new(26, 31),
                                identifier: Ident::new("a", (26, 27)).into(),
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
