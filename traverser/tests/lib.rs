#[macro_export]
macro_rules! assert_traverse_trace {
    (
        source: $source:literal,
        assert_trace: [
            $( $fn_name:literal, )*
        ]
    ) => {
        use fajt_lexer::Lexer;
        use fajt_common::io::PeekReader;
        use fajt_parser::Parser;
        use fajt_ast::SourceType;
        use fajt_traverser::TraceVisitor;
        use fajt_traverser::Fold;

        // TODO abstract this, parsing to ast should be just a function call from fajt_parser.
        let lexer = Lexer::new($source).unwrap();
        let mut reader = PeekReader::new(lexer).unwrap();
        let ast = Parser::parse::<fajt_ast::Program>(&mut reader, SourceType::Script).unwrap();

        let mut visitor = TraceVisitor { visits: vec![] };
        ast.fold(&mut visitor);

        let expected_visits: Vec<&str> = vec![ $( $fn_name, )* ];
        assert_eq!(visitor.visits, expected_visits);
    }
}
