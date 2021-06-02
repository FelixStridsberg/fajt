use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn await_() {
    parser_test!(
        input: "await a",
        expr_output: [
            AwaitExpression {
                span: Span::new(0, 7),
                argument: IdentifierReference::Ident(Ident::new("a", (6, 7))).into(),
            }.into()
        ]
    );
}
