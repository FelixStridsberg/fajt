#[macro_export]
macro_rules! parse {
    (stmt: $input:expr) => {{
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let mut reader = fajt_common::io::PeekReader::new(lexer).unwrap();
        let mut parser = fajt_parser::Parser::new(&mut reader).unwrap();

        parser.parse_statement()
    }};
    (expr: $input:expr) => {{
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let mut reader = fajt_common::io::PeekReader::new(lexer).unwrap();
        let mut parser = fajt_parser::Parser::new(&mut reader).unwrap();

        parser.parse_expression()
    }};
    (program: $input:expr) => {{
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let mut reader = fajt_common::io::PeekReader::new(lexer).unwrap();
        let mut parser = fajt_parser::Parser::new(&mut reader).unwrap();

        parser.parse()
    }};
    (expr: $input:literal) => {{
        parse!(expr: $input, context: fajt_parser::ContextModify::default())
    }};
    (expr: $input:literal, context: $context:expr) => {{
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let mut reader = fajt_common::io::PeekReader::new(lexer).unwrap();
        let mut parser = fajt_parser::Parser::new(&mut reader).unwrap();

        parser.with_context(&$context).parse_expression()
    }};
    ($input:literal) => {{
        parse!($input, context: fajt_parser::ContextModify::default())
    }};
    ($input:literal, context: $context:expr) => {{
        let lexer = fajt_lexer::Lexer::new(&$input).unwrap();
        let mut reader = fajt_common::io::PeekReader::new(lexer).unwrap();
        let mut parser = fajt_parser::Parser::new(&mut reader).unwrap();
        parser.with_context(&$context).parse()
    }};
}
