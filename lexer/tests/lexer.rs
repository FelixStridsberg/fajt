#[test]
fn first_on_line() {
    let input = "async function\nasync function";
    let mut lexer = fajt_lexer::Lexer::new(input).unwrap();
    let tokens = lexer.read_all().unwrap();

    assert_eq!(tokens[0].first_on_line, true);
    assert_eq!(tokens[1].first_on_line, false);
    assert_eq!(tokens[2].first_on_line, true);
    assert_eq!(tokens[3].first_on_line, false);
}
