use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_yield() {
    parser_test!(
        input: "yield",
        expr_output: [
            YieldExpression {
                span: Span::new(0, 5),
                argument: None,
                delegate: false,
            }.into()
        ]
    );
}
