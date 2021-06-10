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
                body: ArrowFunctionBody::Block(vec![])
            }.into()
        ]
    );
}

#[test]
fn identifier_argument_expression_body() {
    parser_test!(
        input: "a => b",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 6),
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
                body: ArrowFunctionBody::Expression(
                    IdentifierReference::Ident(Ident::new("b", (5, 6))).into()
                )
            }.into()
        ]
    );
}

#[test]
fn no_arguments() {
    parser_test!(
        input: "() => {}",
        expr_output: [
            ArrowFunctionExpression {
                span: Span::new(0, 8),
                binding_parameter: true,
                parameters: None,
                body: ArrowFunctionBody::Block(vec![])
            }.into()
        ]
    );
}
