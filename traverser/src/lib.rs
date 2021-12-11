use fajt_ast::{
    BindingElement, Body, DeclFunction, Expr, ExprBinary, FormalParameters, Ident, Program,
    StatementList, Stmt, StmtExpr,
};

trait Fold {
    fn fold(self) -> Self;
}

impl Fold for Ident {
    fn fold(self) -> Self {
        println!("Traverse: Ident {}", self.name);
        self
    }
}

impl Fold for BindingElement {
    fn fold(self) -> Self {
        println!("Traverse: Binding element {:?}", self.pattern);
        self
    }
}

impl Fold for FormalParameters {
    fn fold(self) -> Self {
        FormalParameters {
            span: self.span,
            bindings: self.bindings.into_iter().map(|b| b.fold()).collect(),
            rest: self.rest,
        }
    }
}

impl Fold for Body {
    fn fold(self) -> Self {
        Body {
            span: self.span,
            directives: self.directives,
            statements: self
                .statements
                .into_iter()
                .map(|stmt| stmt.fold())
                .collect(),
        }
    }
}

impl Fold for DeclFunction {
    fn fold(self) -> Self {
        println!("Traverse: DeclFunction {}", self.identifier.name);

        DeclFunction {
            span: self.span,
            asynchronous: self.asynchronous,
            generator: self.generator,
            identifier: self.identifier.fold(),
            parameters: self.parameters.fold(),
            body: self.body.fold(),
        }
    }
}

impl Fold for StmtExpr {
    fn fold(self) -> Self {
        StmtExpr {
            span: self.span,
            expr: self.expr.fold(),
        }
    }
}

impl Fold for ExprBinary {
    fn fold(self) -> Self {
        ExprBinary {
            span: self.span,
            operator: self.operator,
            left: Box::new(self.left.fold()),
            right: Box::new(self.right.fold()),
        }
    }
}

impl Fold for Expr {
    fn fold(self) -> Self {
        return match self {
            Expr::Binary(expr) => Expr::Binary(expr.fold()),
            Expr::IdentRef(expr) => Expr::IdentRef(expr.fold()),
            x => {
                println!("Fold not implemented for expr: {:?}", x);
                x
            }
        };
    }
}

impl Fold for Stmt {
    fn fold(self) -> Self {
        println!("Traverse: Stmt");
        return match self {
            Stmt::FunctionDecl(stmt) => Stmt::FunctionDecl(stmt.fold()),
            Stmt::Expr(stmt) => Stmt::Expr(stmt.fold()),
            x => {
                println!("Fold not implemented for stmt: {:?}", x);
                x
            }
        };
    }
}

impl Fold for StatementList<Stmt> {
    fn fold(self) -> Self {
        println!("Traverse: StmtList");
        StatementList {
            span: self.span,
            body: self.body.into_iter().map(|stmt| stmt.fold()).collect(),
        }
    }
}

impl Fold for Program {
    fn fold(self) -> Self {
        match self {
            Program::Script(stmts) | Program::Module(stmts) => Program::Script(stmts.fold()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Fold;
    use fajt_ast::{
        BinaryOperator, BindingElement, Body, DeclFunction, ExprBinary, FormalParameters, Ident,
        Program, Span, StatementList, Stmt, StmtExpr,
    };

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

        let _ast2 = ast.fold();

        assert_eq!(1, 2);
    }
}
