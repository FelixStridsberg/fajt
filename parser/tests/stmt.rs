mod lib;

use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn parse_empty_statement() {
    parser_test!(
        input: ";",
        output: [
            EmptyStmt {
                span: Span::new(0, 1)
            }.into()
        ]
    );
}
