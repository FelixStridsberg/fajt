use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn prefix_increase() {
    parser_test!(
        input: "++a",
        expr_output: [
            ExprUpdate {
                span: Span::new(0, 3),
                operator: UpdateOperator::Increase,
                prefix: true,
                argument: Ident::new("a", (2, 3)).into(),
            }.into()
        ]
    );
}

#[test]
fn prefix_decrease() {
    parser_test!(
        input: "--a",
        expr_output: [
            ExprUpdate {
                span: Span::new(0, 3),
                operator: UpdateOperator::Decrease,
                prefix: true,
                argument: Ident::new("a", (2, 3)).into(),
            }.into()
        ]
    );
}

#[test]
fn suffix_increase() {
    parser_test!(
        input: "a++",
        expr_output: [
            ExprUpdate {
                span: Span::new(0, 3),
                operator: UpdateOperator::Increase,
                prefix: false,
                argument: Ident::new("a", (0, 1)).into(),
            }.into()
        ]
    );
}

#[test]
fn suffix_decrease() {
    parser_test!(
        input: "a--",
        expr_output: [
            ExprUpdate {
                span: Span::new(0, 3),
                operator: UpdateOperator::Decrease,
                prefix: false,
                argument: Ident::new("a", (0, 1)).into(),
            }.into()
        ]
    );
}

#[test]
fn suffix_increase_no_new_line() {
    let a = parse!(expr: "a\n++").unwrap();
    assert!(!matches!(a, Expr::Update(_)))
}

#[test]
fn suffix_decrease_no_new_line() {
    let a = parse!(expr: "a\n--").unwrap();
    assert!(!matches!(a, Expr::Update(_)))
}
