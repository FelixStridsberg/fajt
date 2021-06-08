use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn identifier_argument() {
    parser_test!(
        input: "a => {}",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 7),
                binding_parameter: true,
                parameters: Some(FormalParameters {
                    span: Span::new(0, 1),
                    parameters: vec![
                        BindingElement {
                            span: Span::new(0, 1),
                            pattern: Ident::new("a", (0, 1)).into(),
                            initializer: None,
                        }
                    ],
                    rest: None,
                }),
            }.into()
        ]
    );
}
