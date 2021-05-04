#[macro_export]
macro_rules! parser_test{
    (input: $input:literal, output:[$($output:expr),*]) => {
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let mut parser = fajt_parser::Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();

        assert_eq!(program, fajt_parser::ast::Program::from_body(vec![$($output),*]))
    };
    (input: $input:literal, error: $error:expr) => {
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let mut parser = fajt_parser::Parser::new(lexer).unwrap();
        let error = parser.parse().unwrap_err();

        assert_eq!(error.kind(), &$error)
    };
}