use fajt_lexer::token::Span;
use fajt_parser::ast::*;
use fajt_parser::error::ErrorKind::SyntaxError;

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
                        kind: ClassMethodKind::Method,
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
                        kind: ClassMethodKind::Method,
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
                        kind: ClassMethodKind::Method,
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

#[test]
fn class_with_empty_async_generator_method() {
    parser_test!(
        input: "class { async *method1() {} }",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 29),
                identifier: None,
                super_class: None,
                body: vec![
                    ClassMethod {
                        span: Span::new(8, 27),
                        name: PropertyName::Ident(Ident::new("method1", (15, 22))),
                        kind: ClassMethodKind::Method,
                        parameters: None,
                        body: vec![],
                        generator: true,
                        asynchronous: true,
                    }.into()
                ],
            }.into()
        ]
    );
}

#[test]
fn class_empty_getter_method() {
    parser_test!(
        input: "class { get getter() {} }",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 25),
                identifier: None,
                super_class: None,
                body: vec![
                    ClassMethod {
                        span: Span::new(8, 23),
                        name: PropertyName::Ident(Ident::new("getter", (12, 18))),
                        kind: ClassMethodKind::Get,
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
fn class_empty_setter_method() {
    parser_test!(
        input: "class { set setter(a) {} }",
        expr_output: [
            ClassExpression {
                span: Span::new(0, 26),
                identifier: None,
                super_class: None,
                body: vec![
                    ClassMethod {
                        span: Span::new(8, 24),
                        name: PropertyName::Ident(Ident::new("setter", (12, 18))),
                        kind: ClassMethodKind::Set,
                        parameters: Some(FormalParameters {
                            span: Span::new(18, 21),
                            bindings: vec![
                                BindingElement {
                                    span: Span::new(19, 20),
                                    pattern: BindingPattern::Ident(Ident::new("a", (19, 20))),
                                    initializer: None,
                                }
                            ],
                            rest: None,
                        }),
                        body: vec![],
                        generator: false,
                        asynchronous: false,
                    }.into()
                ],
            }.into()
        ]
    );
}

#[ignore] // TODO
#[test]
fn fail_setter_method_multiple_parameters() {
    parser_test!(
        input: "class { set setter(a, b) {} }",
        expr_error: SyntaxError(
            "Setter must have exactly one formal parameter.".to_owned(),
            Span::new(18, 24)
        )
    );
}

#[ignore] // TODO
#[test]
fn fail_setter_method_without_parameter() {
    parser_test!(
        input: "class { set setter() {} }",
        expr_error: SyntaxError(
            "Setter must have exactly one formal parameter.".to_owned(),
            Span::new(18, 20)
        )
    );
}

#[ignore] // TODO
#[test]
fn fail_getter_method_with_parameter() {
    parser_test!(
        input: "class { set setter(a) {} }",
        expr_error: SyntaxError(
            "Getter must not have any formal parameters.".to_owned(),
            Span::new(18, 20)
        )
    );
}
