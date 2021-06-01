use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn plus() {
    parser_test!(
        input: "+a",
        expr_output: [
            UnaryExpression {
                span: Span::new(0, 2),
                operator: UnaryOperator::Plus,
                argument: IdentifierReference::Ident(Ident::new("a", (1, 2))).into(),
            }.into()
        ]
    );
}

#[test]
fn minus() {
    parser_test!(
        input: "-a",
        expr_output: [
            UnaryExpression {
                span: Span::new(0, 2),
                operator: UnaryOperator::Minus,
                argument: IdentifierReference::Ident(Ident::new("a", (1, 2))).into(),
            }.into()
        ]
    );
}

#[test]
fn bitwise_not() {
    parser_test!(
        input: "~a",
        expr_output: [
            UnaryExpression {
                span: Span::new(0, 2),
                operator: UnaryOperator::BitwiseNot,
                argument: IdentifierReference::Ident(Ident::new("a", (1, 2))).into(),
            }.into()
        ]
    );
}

#[test]
fn not() {
    parser_test!(
        input: "!a",
        expr_output: [
            UnaryExpression {
                span: Span::new(0, 2),
                operator: UnaryOperator::Not,
                argument: IdentifierReference::Ident(Ident::new("a", (1, 2))).into(),
            }.into()
        ]
    );
}
