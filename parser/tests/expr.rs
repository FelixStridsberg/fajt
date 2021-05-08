mod lib;

use fajt_parser::ast::*;

#[test]
fn this() {
    parser_test!(
        input: "this",
        output: [Expr::This(ThisExpr::new((0, 4))).into()]
    );
}

#[test]
fn null_literal() {
    parser_test!(
        input: "null",
        output: [Expr::Literal(LiteralExpr::new(Literal::Null, (0, 4))).into()]
    );
}
