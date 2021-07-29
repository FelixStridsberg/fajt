use fajt_lexer::token::Span;
use fajt_parser::ast::*;
use fajt_parser::ContextModify;

#[test]
fn await_as_identifier() {
    parser_test!(
        input: "await",
        expr_output: [
            Ident::new("await", (0, 5)).into()
        ]
    );
}

#[test]
fn await_() {
    parser_test!(
        input: "await a",
        context: ContextModify::new().set_await(true),
        expr_output: [
            ExprAwait {
                span: Span::new(0, 7),
                argument: ExprIdentifier::Ident(Ident::new("a", (6, 7))).into(),
            }.into()
        ]
    );
}
