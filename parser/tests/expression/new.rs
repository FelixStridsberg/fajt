use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn new() {
    parser_test!(
        input: "new a",
        expr_output: [
            NewExpression {
                span: Span::new(0, 5),
                callee: IdentifierReference::Ident(Ident::new("a", (4, 5))).into(),
            }.into()
        ]
    );
}

#[test]
fn new_nested() {
    parser_test!(
        input: "new new a",
        expr_output: [
            NewExpression {
                span: Span::new(0, 9),
                callee: NewExpression {
                    span: Span::new(4, 9),
                    callee: IdentifierReference::Ident(Ident::new("a", (8, 9))).into(),
                }.into(),
            }.into()
        ]
    );
}
