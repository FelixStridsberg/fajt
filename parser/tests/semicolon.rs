mod r#macro;

use fajt_lexer::token::Span;
use fajt_parser::ast::{Ident, StmtExpr};

#[test]
fn expr_stmt_with_semicolon() {
    parser_test!(
        input: "a;",
        output: [
            StmtExpr {
                span: Span::new(0, 2),
                expr: Ident::new("a", (0, 1)).into(),
            }.into()
        ]
    );
}

#[test]
fn expr_stmt_without_semicolon() {
    parser_test!(
        input: "a",
        output: [
            StmtExpr {
                span: Span::new(0, 1),
                expr: Ident::new("a", (0, 1)).into(),
            }.into()
        ]
    );
}
