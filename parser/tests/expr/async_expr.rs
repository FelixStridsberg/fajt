use fajt_lexer::token::Span;
use fajt_parser::ast::*;
use fajt_parser::ContextModify;

// TODO this is not supported by snapshot tests, probably create the context by js?
#[test]
fn r#await() {
    parser_test!(
        input: "await a",
        context: ContextModify::new().set_await(true),
        expr_output: [
            ExprAwait {
                span: Span::new(0, 7),
                argument: Ident::new("a", (6, 7)).into(),
            }.into()
        ]
    );
}
