use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn conditional() {
    parser_test!(
        input: "test ? consequent : alternate",
        expr_output: [
            ExprConditional {
                span: Span::new(0, 29),
                condition: Ident::new("test", (0, 4)).into(),
                consequent: Ident::new("consequent", (7, 17)).into(),
                alternate: Ident::new("alternate", (20, 29)).into(),
            }.into()
        ]
    );
}
