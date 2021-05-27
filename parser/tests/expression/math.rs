use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn addition() {
    parser_test!(
        input: "1 + 1",
        output: [
            // TODO
        ]
    );
}