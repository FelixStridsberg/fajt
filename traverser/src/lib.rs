#[macro_use]
mod fold;

use fajt_ast::{
    BindingElement, Body, DeclFunction, Expr, ExprBinary, FormalParameters, Ident, Program,
    StatementList, Stmt, StmtExpr,
};

trait Visitor {
    fn visit_ident(&mut self, ident: Ident) -> Ident {
        ident
    }
}

trait Fold {
    fn fold(self, visitor: &mut dyn Visitor) -> Self;
}

impl<F: Fold> Fold for Vec<F> {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        self.into_iter().map(|item| item.fold(visitor)).collect()
    }
}

impl<F: Fold> Fold for Box<F> {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        Box::new((*self).fold(visitor))
    }
}

generate_fold! {
    enum Program {
        Script
        Module
    }

    enum Expr {
        Binary
        IdentRef
    }

    enum Stmt {
        FunctionDecl
        Expr
    }
}

generate_fold! {
    Body {
        statements
    }

    DeclFunction {
        identifier
        parameters
        body
    }

    StmtExpr {
        expr
    }

    ExprBinary {
        left
        right
    }

    StatementList<Stmt> {
        body
    }

    BindingElement { }

    FormalParameters {
        bindings
    }

    Ident (visit: visit_ident) {}
}

#[cfg(test)]
mod tests {
    use super::Fold;
    use crate::Visitor;
    use fajt_ast::{
        BinaryOperator, BindingElement, Body, DeclFunction, ExprBinary, FormalParameters, Ident,
        Program, Span, StatementList, Stmt, StmtExpr,
    };

    struct IdentModifier {}

    impl Visitor for IdentModifier {
        fn visit_ident(&mut self, mut ident: Ident) -> Ident {
            println!("VISITED: {:?}", ident);
            ident.name = "Other name".to_string();
            ident
        }
    }

    #[test]
    fn it_works() {
        let ast = Program::Module(StatementList {
            span: Span::new(0, 0),
            body: vec![DeclFunction {
                span: Span::new(0, 0),
                asynchronous: false,
                generator: false,
                identifier: Ident::new("square", (0, 0)),
                parameters: FormalParameters {
                    span: Span::new(0, 0),
                    bindings: vec![BindingElement {
                        span: Span::new(0, 0),
                        pattern: Ident::new("n", (0, 0)).into(),
                        initializer: None,
                    }],
                    rest: None,
                },
                body: Body {
                    span: Span::new(0, 0),
                    directives: vec![],
                    statements: vec![Stmt::Expr(StmtExpr {
                        span: Span::new(0, 0),
                        expr: ExprBinary {
                            span: Span::new(0, 0),
                            operator: BinaryOperator::Multiplication,
                            left: Box::new(Ident::new("n", (0, 0)).into()),
                            right: Box::new(Ident::new("n", (0, 0)).into()),
                        }
                        .into(),
                    })],
                },
            }
            .into()],
        });

        let mut visitor = IdentModifier {};
        let ast2 = ast.fold(&mut visitor);

        println!("AST2 {:?}", ast2);

        assert_eq!(1, 2);
    }
}
