use fajt_ast::{
    BindingElement, Body, DeclFunction, Expr, ExprBinary, FormalParameters, Ident, Program,
    StatementList, Stmt, StmtExpr,
};

trait Fold {
    fn fold(self, visitor: &mut dyn Visitor) -> Self;
}


trait Visitor {
    fn visit_ident(&mut self, ident: Ident) -> Ident {
        ident
    }
}

struct IdentModifier {}

impl Visitor for IdentModifier {
    fn visit_ident(&mut self, mut ident: Ident) -> Ident {
        println!("VISITED: {:?}", ident);
        ident.name = "Other name".to_string();
        ident
    }
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
            bindings: self.bindings.fold(visitor),
            ..self
        }
    }
}

macro_rules! generate_fold {
    (
        $(
            $ident:ident $( <$param:ident> )? {
                $(
                    $field: ident
                )*
            }
        )*
    ) => {
        $(
            impl Fold for $ident $( <$param> )? {
                fn fold(self, visitor: &mut dyn Visitor) -> Self {
                    println!("Visit: {}", stringify!($ident));
                    $ident {
                        $($field: self.$field.fold(visitor),)*
                        ..self
                    }
                }
            }
        )*
    };

    (
        $(
            enum $ident:ident {
                $(
                    $field: ident
                )*
            }
        )*
    ) => {
        $(
            impl Fold for $ident {
                fn fold(self, visitor: &mut dyn Visitor) -> Self {
                    match self {
                        $( $ident::$field(v) => $ident::$field(v.fold(visitor)), )*
                        _ => self
                    }
                }
            }
        )*
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

#[cfg(test)]
mod tests {
    use super::Fold;
    use crate::IdentModifier;
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

        let mut visitor = IdentModifier {};
        let ast2 = ast.fold(&mut visitor);

        //println!("AST2 {:?}", ast2);

        assert_eq!(1, 2);
    }
}
