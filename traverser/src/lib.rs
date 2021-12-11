use fajt_ast::{
    BindingElement, Body, DeclFunction, Expr, ExprBinary, FormalParameters, Ident, Program,
    StatementList, Stmt, StmtExpr,
};

trait Fold {
    fn fold(self, visitor: &mut dyn Visitor) -> Self;
}


trait Visitor {
    fn visit_ident(&mut self, ident: Ident) -> Ident;
}

struct IdentModifier {}

impl Visitor for IdentModifier {
    fn visit_ident(&mut self, mut ident: Ident) -> Ident {
        println!("VISITED: {:?}", ident);
        ident.name = "Other name".to_string();
        ident
    }
}

impl Fold for Ident {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        println!("Traverse: Ident {}", self.name);
        visitor.visit_ident(self)
    }
}

impl Fold for BindingElement {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        println!("Traverse: Binding element {:?}", self.pattern);
        self
    }
}

impl Fold for FormalParameters {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        // const modified = modify(self);
        FormalParameters {
            span: self.span,
            bindings: self.bindings.into_iter().map(|b| b.fold(visitor)).collect(),
            rest: self.rest,
        }
    }
}

impl Fold for Body {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        Body {
            span: self.span,
            directives: self.directives,
            statements: self
                .statements
                .into_iter()
                .map(|stmt| stmt.fold(visitor))
                .collect(),
        }
    }
}

impl Fold for DeclFunction {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        println!("Traverse: DeclFunction {}", self.identifier.name);

        DeclFunction {
            span: self.span,
            asynchronous: self.asynchronous,
            generator: self.generator,
            identifier: self.identifier.fold(visitor),
            parameters: self.parameters.fold(visitor),
            body: self.body.fold(visitor),
        }
    }
}

impl Fold for StmtExpr {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        StmtExpr {
            span: self.span,
            expr: self.expr.fold(visitor),
        }
    }
}

impl Fold for ExprBinary {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        ExprBinary {
            span: self.span,
            operator: self.operator,
            left: Box::new(self.left.fold(visitor)),
            right: Box::new(self.right.fold(visitor)),
        }
    }
}

impl Fold for Expr {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        return match self {
            Expr::Binary(expr) => Expr::Binary(expr.fold(visitor)),
            Expr::IdentRef(expr) => Expr::IdentRef(expr.fold(visitor)),
            x => {
                println!("Fold not implemented for expr: {:?}", x);
                x
            }
        };
    }
}

impl Fold for Stmt {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        println!("Traverse: Stmt");
        return match self {
            Stmt::FunctionDecl(stmt) => Stmt::FunctionDecl(stmt.fold(visitor)),
            Stmt::Expr(stmt) => Stmt::Expr(stmt.fold(visitor)),
            x => {
                println!("Fold not implemented for stmt: {:?}", x);
                x
            }
        };
    }
}

impl Fold for StatementList<Stmt> {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        println!("Traverse: StmtList");
        StatementList {
            span: self.span,
            body: self.body.into_iter().map(|stmt| stmt.fold(visitor)).collect(),
        }
    }
}

impl Fold for Program {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        match self {
            Program::Script(stmts) | Program::Module(stmts) => Program::Script(stmts.fold(visitor)),
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
    use crate::IdentModifier;

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
