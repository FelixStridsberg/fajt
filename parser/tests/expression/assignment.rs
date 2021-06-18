use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn assign() {
    parser_test!(
        input: "a = b",
        expr_output: [
            AssignmentExpression {
                span: Span::new(0, 5),
                operator: AssignmentOperator::Assign,
                left: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                right: IdentifierReference::Ident(Ident::new("b", (4, 5))).into(),
            }.into()
        ]
    );
}
