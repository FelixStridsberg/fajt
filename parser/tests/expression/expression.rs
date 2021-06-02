use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn sequence_expression() {
    parser_test!(
        input: "a, b, c",
        expr_output: [
            SequenceExpression {
                span: Span::new(0, 7),
                expressions: vec![
                    Ident::new("a", (0, 1)).into(),
                    Ident::new("b", (3, 4)).into(),
                    Ident::new("c", (6, 7)).into(),
                ]
            }.into()
        ]
    );
}
