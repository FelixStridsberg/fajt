mod lib;

use fajt_lexer::punct;
use fajt_lexer::token;
use fajt_lexer::token::{Span, Token};
use fajt_parser::ast::Expression::IdentifierReference;
use fajt_parser::ast::VariableKind::*;
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::UnexpectedToken;

#[test]
fn no_initializer() {
    parser_test!(
        input: "var foo;",
        output: [
            VariableStatement {
                span: Span::new(0, 8),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 8),
                        identifier: Ident::new("foo", (4, 7)).into(),
                        initializer: None,
                    },
                ]
            }.into()
        ]
    );
}

#[test]
fn with_initializer() {
    parser_test!(
        input: "var foo = a;",
        output: [
            VariableStatement {
                span: Span::new(0, 12),
                kind: Var,
                declarations: vec![
                    VariableDeclaration {
                        span: Span::new(4, 12),
                        identifier: Ident::new("foo", (4, 7)).into(),
                        initializer: Some(IdentifierReference(Ident::new("a", (10, 11)).into()))
                    }
                ]
            }.into()
        ]
    );
}

#[test]
fn fail_invalid_identifier() {
    parser_test!(
        input: "var * = c;",
        error: UnexpectedToken(Token::new(punct!("*"), false, (4, 5)))
    );
}
