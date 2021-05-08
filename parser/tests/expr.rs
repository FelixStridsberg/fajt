mod lib;

use fajt_parser::ast::*;

#[test]
fn primary_expression_this() {
    parser_test!(
        input: "this",
        output: [Expr::This(ThisExpr::new((0, 4))).into()]
    );
}
