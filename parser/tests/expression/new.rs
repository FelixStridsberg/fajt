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
