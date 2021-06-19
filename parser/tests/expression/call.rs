use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn super_call() {
    parser_test!(
        input: "super()",
        expr_output: [
            CallExpression {
                span: Span::new(0, 7),
                callee: Callee::Super,
                arguments_span: Span::new(5, 7),
                arguments: vec![]
            }.into()
        ]
    );
}

#[test]
fn super_call_with_args() {
    parser_test!(
        input: "super(a, b)",
        expr_output: [
            CallExpression {
                span: Span::new(0, 11),
                callee: Callee::Super,
                arguments_span: Span::new(5, 11),
                arguments: vec![
                    Argument::Expression(Ident::new("a", (6, 7)).into()),
                    Argument::Expression(Ident::new("b", (9, 10)).into()),
                ]
            }.into()
        ]
    );
}
