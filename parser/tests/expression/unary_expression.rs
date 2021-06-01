use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn addition() {
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
