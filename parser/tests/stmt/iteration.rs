use fajt_lexer::token::Span;
use fajt_parser::ast::{
    BindingPattern, ExprLiteral, ForInit, Ident, Literal, Stmt, StmtDoWhile, StmtEmpty, StmtFor,
    StmtForIn, StmtForOf, StmtVariable, StmtWhile, VariableDeclaration, VariableKind,
};
use fajt_parser::error::ErrorKind::SyntaxError;
use fajt_parser::ContextModify;

#[test]
fn do_while() {
    parser_test!(
        input: "do a; while (true)",
        output: [
            StmtDoWhile {
                span: Span::new(0, 18),
                body: Stmt::expr(Ident::new("a", (3, 4))).into(),
                test: ExprLiteral {
                    span: Span::new(13, 17),
                    literal: Literal::Boolean(true),
                }.into()
            }.into()
        ]
    );
}

#[test]
fn r#while() {
    parser_test!(
        input: "while (true) a",
        output: [
            StmtWhile {
                span: Span::new(0, 14),
                test: ExprLiteral {
                    span: Span::new(7, 11),
                    literal: Literal::Boolean(true),
                }.into(),
                body: Stmt::expr(Ident::new("a", (13, 14))).into(),
            }.into()
        ]
    );
}

#[test]
fn empty_for() {
    parser_test!(
        input: "for (;;) ;",
        output: [
            StmtFor {
                span: Span::new(0, 10),
                init: None,
                test: None,
                update: None,
                body: StmtEmpty {
                   span: Span::new(9, 10),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_with_init() {
    parser_test!(
        input: "for (a;;) ;",
        output: [
            StmtFor {
                span: Span::new(0, 11),
                init: Some(ForInit::Expression(Ident::new("a", (5, 6)).into())),
                test: None,
                update: None,
                body: StmtEmpty {
                   span: Span::new(10, 11),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_with_test() {
    parser_test!(
        input: "for (;a;) ;",
        output: [
            StmtFor {
                span: Span::new(0, 11),
                init: None,
                test: Some(Ident::new("a", (6, 7)).into()),
                update: None,
                body: StmtEmpty {
                   span: Span::new(10, 11),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_with_update() {
    parser_test!(
        input: "for (;;a) ;",
        output: [
            StmtFor {
                span: Span::new(0, 11),
                init: None,
                test: None,
                update: Some(Ident::new("a", (7, 8)).into()),
                body: StmtEmpty {
                   span: Span::new(10, 11),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_with_var_declaration() {
    parser_test!(
        input: "for (var a;;) ;",
        output: [
            StmtFor {
                span: Span::new(0, 15),
                init: Some(ForInit::Declaration(
                    StmtVariable {
                        span: Span::new(5, 10),
                        kind: VariableKind::Var,
                        declarations: vec![
                            VariableDeclaration {
                                span: Span::new(9, 10),
                                pattern: BindingPattern::Ident(Ident::new("a", (9, 10))),
                                initializer: None,
                            }
                        ]
                    }
                )),
                test: None,
                update: None,
                body: StmtEmpty {
                   span: Span::new(14, 15),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_with_let_declaration() {
    parser_test!(
        input: "for (let a;;) ;",
        output: [
            StmtFor {
                span: Span::new(0, 15),
                init: Some(ForInit::Declaration(
                    StmtVariable {
                        span: Span::new(5, 10),
                        kind: VariableKind::Let,
                        declarations: vec![
                            VariableDeclaration {
                                span: Span::new(9, 10),
                                pattern: BindingPattern::Ident(Ident::new("a", (9, 10))),
                                initializer: None,
                            }
                        ]
                    }
                )),
                test: None,
                update: None,
                body: StmtEmpty {
                   span: Span::new(14, 15),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_with_const_declaration() {
    parser_test!(
        input: "for (const a;;) ;",
        output: [
            StmtFor {
                span: Span::new(0, 17),
                init: Some(ForInit::Declaration(
                    StmtVariable {
                        span: Span::new(5, 12),
                        kind: VariableKind::Const,
                        declarations: vec![
                            VariableDeclaration {
                                span: Span::new(11, 12),
                                pattern: BindingPattern::Ident(Ident::new("a", (11, 12))),
                                initializer: None,
                            }
                        ]
                    }
                )),
                test: None,
                update: None,
                body: StmtEmpty {
                   span: Span::new(16, 17),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_in() {
    parser_test!(
        input: "for (a in b) ;",
        output: [
            StmtForIn {
                span: Span::new(0, 14),
                left: ForInit::Expression(Ident::new("a", (5, 6)).into()),
                right: Ident::new("b", (10, 11)).into(),
                body: StmtEmpty {
                   span: Span::new(13, 14),
                }.into(),
            }.into()
        ]
    );
}

#[test]
fn for_in_with_declaration() {
    parser_test!(
        input: "for (var a in b) ;",
        output: [
            StmtForIn {
                span: Span::new(0, 18),
                left: ForInit::Declaration(StmtVariable {
                    span: Span::new(5, 10),
                    kind: VariableKind::Var,
                    declarations: vec![
                        VariableDeclaration {
                            span: Span::new(9, 10),
                            pattern: Ident::new("a", (9, 10)).into(),
                            initializer: None,
                        }
                    ]
                }),
                right: Ident::new("b", (14, 15)).into(),
                body: StmtEmpty {
                   span: Span::new(17, 18),
                }.into(),
            }.into()
        ]
    );
}

#[test]
#[ignore] // TODO
fn fail_for_in_with_multiple_declarations() {
    parser_test!(
        input: "for (var a, b in c) ;",
        error: SyntaxError(
            "Invalid left-hand side in for-in loop: Must have a single binding.".to_owned(),
            Span::new(5, 13)
        )
    );
}

#[test]
fn for_of() {
    parser_test!(
        input: "for (a of b) ;",
        output: [
            StmtForOf {
                span: Span::new(0, 14),
                left: ForInit::Expression(Ident::new("a", (5, 6)).into()),
                right: Ident::new("b", (10, 11)).into(),
                body: StmtEmpty {
                   span: Span::new(13, 14),
                }.into(),
                wait: false,
            }.into()
        ]
    );
}

#[test]
fn for_await_of() {
    parser_test!(
        input: "for await (a of b) ;",
        context: ContextModify::new().set_await(true),
        output: [
            StmtForOf {
                span: Span::new(0, 20),
                left: ForInit::Expression(Ident::new("a", (11, 12)).into()),
                right: Ident::new("b", (16, 17)).into(),
                body: StmtEmpty {
                   span: Span::new(19, 20),
                }.into(),
                wait: true,
            }.into()
        ]
    );
}

#[test]
fn for_of_with_declaration() {
    parser_test!(
        input: "for (var a of b) ;",
        output: [
            StmtForOf {
                span: Span::new(0, 18),
                left: ForInit::Declaration(StmtVariable {
                    span: Span::new(5, 10),
                    kind: VariableKind::Var,
                    declarations: vec![
                        VariableDeclaration {
                            span: Span::new(9, 10),
                            pattern: Ident::new("a", (9, 10)).into(),
                            initializer: None,
                        }
                    ]
                }),
                right: Ident::new("b", (14, 15)).into(),
                body: StmtEmpty {
                   span: Span::new(17, 18),
                }.into(),
                wait: false,
            }.into()
        ]
    );
}

#[test]
#[ignore] // TODO
fn fail_for_of_with_multiple_declarations() {
    parser_test!(
        input: "for (var a, b of c) ;",
        error: SyntaxError(
            "Invalid left-hand side in for-in loop: Must have a single binding.".to_owned(),
            Span::new(5, 13)
        )
    );
}
