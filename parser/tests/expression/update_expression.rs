use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn prefix_increase() {
    parser_test!(
        input: "++a",
        expr_output: [
            UpdateExpression {
                span: Span::new(0, 3),
                operator: UpdateOperator::Increase,
                prefix: true,
                argument: IdentifierReference::Ident(Ident::new("a", (2, 3))).into(),
            }.into()
        ]
    );
}
