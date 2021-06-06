use fajt_lexer::token::Span;
use fajt_parser::ast::*;

#[test]
fn empty_class() {
    parser_test!(
        input: "class MyClass {}",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 16),
                identifier: Some(Ident::new("MyClass", (6, 13))),
                super_class: None,
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn anonymous_empty_class() {
    parser_test!(
        input: "class {}",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 8),
                identifier: None,
                super_class: None,
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn empty_class_extends() {
    parser_test!(
        input: "class MyClass extends SuperClass {}",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 35),
                identifier: Some(Ident::new("MyClass", (6, 13))),
                super_class: Some(Ident::new("SuperClass", (22, 32)).into()),
                body: vec![],
            }.into()
        ]
    );
}

#[test]
fn class_with_empty_method() {
    parser_test!(
        input: "class { method1() {} }",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 22),
                identifier: None,
                super_class: None,
                body: vec![
                    ClassMethod {
                        span: Span::new(8, 20),
                        name: PropertyName::Ident(Ident::new("method1", (8, 15))),
                        parameters: None,
                        body: vec![],
                        generator: false,
                        asynchronous: false,
                    }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn class_with_empty_generator_method() {
    parser_test!(
        input: "class { *method1() {} }",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 23),
                identifier: None,
                super_class: None,
                body: vec![
                    ClassMethod {
                        span: Span::new(8, 21),
                        name: PropertyName::Ident(Ident::new("method1", (9, 16))),
                        parameters: None,
                        body: vec![],
                        generator: true,
                        asynchronous: false,
                    }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn class_with_empty_async_method() {
    parser_test!(
        input: "class { async method1() {} }",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 28),
                identifier: None,
                super_class: None,
                body: vec![
                    ClassMethod {
                        span: Span::new(8, 26),
                        name: PropertyName::Ident(Ident::new("method1", (14, 21))),
                        parameters: None,
                        body: vec![],
                        generator: false,
                        asynchronous: true,
                    }.into()
                ],
            }.into()
        ]
    );
}
