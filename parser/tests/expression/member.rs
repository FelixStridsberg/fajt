use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn identifier() {
    parser_test!(
        input: "a.b",
        expr_output: [
            MemberExpression {
                span: Span::new(0, 3),
                object: IdentifierReference::Ident(Ident::new("a", (0, 1))).into(),
                member: Member::Ident(Ident::new("b", (2, 3))),
            }.into()
        ]
    );
}
