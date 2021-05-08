mod lib;

use fajt_parser::ast::*;

#[test]
fn parse_empty_statement() {
    parser_test!(
        input: ";",
        output: [EmptyStmt::new((0, 1).into()).into()]
    );
}
