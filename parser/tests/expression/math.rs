use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn addition() {
    parser_test!(
        input: "a + b",
        expr_output: [
            BinaryExpression {
                span: Span::new(0, 5),
                operator: BinaryOperator::Plus,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
            }.into()
        ]
    );
}
