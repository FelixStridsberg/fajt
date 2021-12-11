#[macro_export]
macro_rules! assert_traverse_trace {
    (
        source: $source:literal,
        assert_trace: [
            $( $fn_name:literal, )*
        ]
    ) => {
        use fajt_parser::{ parse_program };
        use fajt_ast::traverse::TraceVisitor;
        use fajt_ast::traverse::Fold;

        let ast = parse_program($source).unwrap();

        let mut visitor = TraceVisitor { visits: vec![] };
        ast.fold(&mut visitor);

        let expected_visits: Vec<&str> = vec![ $( $fn_name, )* ];
        assert_eq!(visitor.visits, expected_visits);
    }
}
