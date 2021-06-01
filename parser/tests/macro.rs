#[macro_export]
macro_rules! parse {
    (expr: $input:literal) => {{
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let mut reader = fajt_common::io::PeekReader::new(lexer).unwrap();
        let mut parser = fajt_parser::Parser::new(&mut reader).unwrap();
        parser.parse_expr()
    }};
    ($input:literal) => {{
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let mut reader = fajt_common::io::PeekReader::new(lexer).unwrap();
        let mut parser = fajt_parser::Parser::new(&mut reader).unwrap();
        parser.parse()
    }};
}

#[macro_export]
macro_rules! parser_test {
    (input: $input:literal, success) => {
        parse!($input).expect("Expected success but got fail.");
    };
    (input: $input:literal, output:[$($output:expr),*]) => {
        let program = parse!($input).unwrap();
        assert_eq!(program, fajt_parser::ast::Program::from_body(vec![$($output),*]))
    };
    (input: $input:literal, expr_output:[$output:expr]) => {
        let expr = parse!(expr: $input).unwrap();
        assert_eq!(expr, $output)
    };
    (input: $input:literal, error: $error:expr) => {
        let error = parse!($input).unwrap_err();
        assert_eq!(error.kind(), &$error)
    };
    (input: $input:literal, expr_error: $error:expr) => {
        let error = parse!(expr: $input).unwrap_err();
        assert_eq!(error.kind(), &$error)
    };
}
