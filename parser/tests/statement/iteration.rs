use fajt_lexer::token::Span;
use fajt_parser::ast::{
    BindingPattern, DoWhileStatement, EmptyStatement, ForInit, ForStatement, Ident, Literal,
    LiteralExpression, Statement, VariableDeclaration, VariableKind, VariableStatement,
    WhileStatement,
};

#[test]
fn do_while() {
    parser_test!(
        input: "do a; while (true)",
        output: [
            DoWhileStatement {
                span: Span::new(0, 18),
                body: Statement::Expression(Ident::new("a", (3, 4)).into()),
                test: LiteralExpression {
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
            WhileStatement {
                span: Span::new(0, 14),
                test: LiteralExpression {
                    span: Span::new(7, 11),
                    literal: Literal::Boolean(true),
                }.into(),
                body: Statement::Expression(Ident::new("a", (13, 14)).into()),
            }.into()
        ]
    );
}

#[test]
fn empty_for() {
    parser_test!(
        input: "for (;;) ;",
        output: [
            ForStatement {
                span: Span::new(0, 10),
                init: None,
                test: None,
                update: None,
                body: EmptyStatement {
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
            ForStatement {
                span: Span::new(0, 11),
                init: Some(ForInit::Expression(Ident::new("a", (5, 6)).into())),
                test: None,
                update: None,
                body: EmptyStatement {
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
            ForStatement {
                span: Span::new(0, 11),
                init: None,
                test: Some(Ident::new("a", (6, 7)).into()),
                update: None,
                body: EmptyStatement {
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
            ForStatement {
                span: Span::new(0, 11),
                init: None,
                test: None,
                update: Some(Ident::new("a", (7, 8)).into()),
                body: EmptyStatement {
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
            ForStatement {
                span: Span::new(0, 15),
                init: Some(ForInit::Declaration(
                    VariableStatement {
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
                body: EmptyStatement {
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
            ForStatement {
                span: Span::new(0, 15),
                init: Some(ForInit::Declaration(
                    VariableStatement {
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
                body: EmptyStatement {
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
            ForStatement {
                span: Span::new(0, 17),
                init: Some(ForInit::Declaration(
                    VariableStatement {
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
                body: EmptyStatement {
                   span: Span::new(16, 17),
                }.into(),
            }.into()
        ]
    );
}
