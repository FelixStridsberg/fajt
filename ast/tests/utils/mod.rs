#[macro_export]
macro_rules! assert_traverse_trace {
    (
        source: $source:literal,
        assert_trace: [
            $( $fn_name:literal, )*
        ]
    ) => {
        use fajt_parser::{ parse_module };
        use fajt_ast::traverse::TraceVisitor;
        use fajt_ast::traverse::Traverse;

        let mut ast = parse_module($source).unwrap();
        let mut visitor = TraceVisitor { visits: vec![] };
        ast.traverse(&mut visitor);

        let expected_visits: Vec<&str> = vec![ $( $fn_name, )* ];
        assert_eq!(visitor.visits, expected_visits);
    }
}
