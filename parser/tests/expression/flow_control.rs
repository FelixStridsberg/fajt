use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn conditional() {
    parser_test!(
        input: "test ? consequent : alternate",
        expr_output: [
            ConditionalExpression {
                span: Span::new(0, 29),
                condition: IdentifierReference::Ident(Ident::new("test", (0, 4))).into(),
                consequent: IdentifierReference::Ident(Ident::new("consequent", (7, 17))).into(),
                alternate: IdentifierReference::Ident(Ident::new("alternate", (20, 29))).into(),
            }.into()
        ]
    );
}
