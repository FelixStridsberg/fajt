use fajt_lexer::token::Span;
use fajt_parser::error::ErrorKind::SyntaxError;

#[ignore] // TODO
#[test]
fn fail_setter_method_multiple_parameters() {
    parser_test!(
        input: "class { set setter(a, b) {} }",
        expr_error: SyntaxError(
            "Setter must have exactly one formal parameter.".to_owned(),
            Span::new(18, 24)
        )
    );
}

#[ignore] // TODO
#[test]
fn fail_setter_method_without_parameter() {
    parser_test!(
        input: "class { set setter() {} }",
        expr_error: SyntaxError(
            "Setter must have exactly one formal parameter.".to_owned(),
            Span::new(18, 20)
        )
    );
}

#[ignore] // TODO
#[test]
fn fail_getter_method_with_parameter() {
    parser_test!(
        input: "class { set setter(a) {} }",
        expr_error: SyntaxError(
            "Getter must not have any formal parameters.".to_owned(),
            Span::new(18, 20)
        )
    );
}
