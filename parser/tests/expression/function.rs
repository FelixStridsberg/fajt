use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_anonymous_function() {
    parser_test!(
        input: "function () {}",
        expr_output: [
            FunctionExpression {
                span: Span::new(0, 14),
            }.into()
        ]
    );
}
