mod lib;

use fajt_parser::ast::*;
use fajt_lexer::token::Span;

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
