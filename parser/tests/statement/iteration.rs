use fajt_lexer::token::Span;
use fajt_parser::ast::{DoWhileStatement, Ident, Literal, LiteralExpression, Statement};

#[test]
fn do_while() {
    parser_test!(
        input: "do a; while (true)",
        output: [
            DoWhileStatement {
                span: Span::new(0, 18),
                body: Statement::Expression(Ident::new("a", (3, 4)).into()),
                test: LiteralExpression {
                    span: Span::new(13, 17),
                    literal: Literal::Boolean(true),
                }.into()
            }.into()
        ]
    );
}
