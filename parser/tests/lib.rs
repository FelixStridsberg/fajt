#[macro_export]
macro_rules! parser_test{
    (input: $input:literal, output:[$($output:expr),*]) => {
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let reader = fajt_parser::Reader::new(lexer);
        let mut parser = fajt_parser::Parser::new(reader);
        let program = parser.parse();

        assert_eq!(program, fajt_parser::ast::Program::from_body(vec![$($output),*]))
    }
}
