/// TODO documentation
#[macro_export]
macro_rules! assert_lexer {
    (input: $input:expr, output: [$(($token:expr, ($col1:expr, $col2:expr)),)*]) => {
        let mut lexer = fajt_lexer::Lexer::new($input).expect("Could not create lexer, empty input?");
        let tokens = lexer.read_all().unwrap();

        assert_eq!(tokens, vec![$(fajt_lexer::token::Token::new($token, ($col1, $col2))),*]);
    };
    (input: $input:expr, error: $error:expr) => {
        let mut lexer = fajt_lexer::Lexer::new($input).expect("Could not create lexer, empty input?");
        let error = lexer.read().expect_err("Expected error but test passed.");

        assert_eq!(error, $error);
    };
}

#[macro_export]
macro_rules! identifier {
    ($name:expr) => {
        fajt_lexer::token::TokenValue::Identifier($name.to_owned())
    };
}
