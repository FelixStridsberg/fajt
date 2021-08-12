use fajt_lexer::token::Span;
use fajt_parser::error::ErrorKind::SyntaxError;

// TODO snapshot fail test
#[test]
fn fail_invalid_optional_chain_from_new_expression() {
    // This is not a NewExpression but a MemberExpression.
    parser_test!(
        input: "new a()?.b",
        success
    );

    parser_test!(
        input: "new new a()?.b",
        expr_error: SyntaxError(
            "Invalid optional chain from new expression".to_owned(),
            Span::new(11, 13)
        )
    );
}
