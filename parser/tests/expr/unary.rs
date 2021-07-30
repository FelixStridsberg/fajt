use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn plus() {
    parser_test!(
        input: "+a",
        expr_output: [
            ExprUnary {
                span: Span::new(0, 2),
                operator: UnaryOperator::Plus,
                argument: Ident::new("a", (1, 2)).into(),
            }.into()
        ]
    );
}

#[test]
fn minus() {
    parser_test!(
        input: "-a",
        expr_output: [
            ExprUnary {
                span: Span::new(0, 2),
                operator: UnaryOperator::Minus,
                argument: Ident::new("a", (1, 2)).into(),
            }.into()
        ]
    );
}

#[test]
fn bitwise_not() {
    parser_test!(
        input: "~a",
        expr_output: [
            ExprUnary {
                span: Span::new(0, 2),
                operator: UnaryOperator::BitwiseNot,
                argument: Ident::new("a", (1, 2)).into(),
            }.into()
        ]
    );
}

#[test]
fn not() {
    parser_test!(
        input: "!a",
        expr_output: [
            ExprUnary {
                span: Span::new(0, 2),
                operator: UnaryOperator::Not,
                argument: Ident::new("a", (1, 2)).into(),
            }.into()
        ]
    );
}

#[test]
fn delete() {
    parser_test!(
        input: "delete a",
        expr_output: [
            ExprUnary {
                span: Span::new(0, 8),
                operator: UnaryOperator::Delete,
                argument: Ident::new("a", (7, 8)).into(),
            }.into()
        ]
    );
}

#[test]
fn void() {
    parser_test!(
        input: "void a",
        expr_output: [
            ExprUnary {
                span: Span::new(0, 6),
                operator: UnaryOperator::Void,
                argument: Ident::new("a", (5, 6)).into(),
            }.into()
        ]
    );
}

#[test]
fn typeof_() {
    parser_test!(
        input: "typeof a",
        expr_output: [
            ExprUnary {
                span: Span::new(0, 8),
                operator: UnaryOperator::Typeof,
                argument: Ident::new("a", (7, 8)).into(),
            }.into()
        ]
    );
}
