/// Test macro for easily running the lexer.
///
/// Assert output:
/// ```
/// assert_lexer!(
///     // String input to the lexer
///     input: "var i = 0;",
///
///     // The output is a tuple of the token and the span.
///     // Use macros for easily creating tokens.
///     output: [
///         (keyword!("var"), (0, 3)),
///         (identifier!("variable"), (4, 12)),
///         (punct!("="), (13, 14)),
///         (literal!(integer, 1), (15, 16)),
///     ]
/// )
/// ```
///
/// Assert error:
/// TODO, syntax not finalized
#[macro_export]
macro_rules! assert_lexer {
    (input: $input:expr, output: [$(($token:expr, ($col1:expr, $col2:expr)),)+]) => {
        let mut lexer = fajt_lexer::Lexer::new($input).expect("Could not create lexer, empty input?");
        let tokens = lexer.read_all().unwrap();

        let mut expected = vec![$(fajt_lexer::token::Token::new($token, false, ($col1, $col2))),*];
        expected[0].first_on_line = true;

        assert_eq!(tokens, expected);
    };
    (input: $input:expr, error: $error:expr) => {
        let mut lexer = fajt_lexer::Lexer::new($input).expect("Could not create lexer, empty input?");
        let error = lexer.read().expect_err("Expected error but test passed.");

        assert_eq!(error, $error);
    };
}

/// Macro for creating identifier, currently there is non in the lexer library, use that one instead
/// if created, not sure if there will be a need for one except for testing.
#[macro_export]
macro_rules! identifier {
    ($name:expr) => {
        fajt_lexer::token::TokenValue::Identifier($name.to_owned())
    };
}
