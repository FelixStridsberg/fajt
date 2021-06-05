use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_class() {
    parser_test!(
        input: "class {}",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 8),
                super_class: None,
                body: vec![],
            }.into()
        ]
    );
}
