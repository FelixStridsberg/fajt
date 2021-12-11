#[macro_use]
mod fold;

use fajt_ast::{
    BindingElement, Body, DeclFunction, Expr, ExprBinary, FormalParameters, Ident, Program,
    StatementList, Stmt, StmtExpr,
};

macro_rules! generate_visit_fn {
    (
        $( $fn_name:ident: $node:ident $( <$param:ident> )?  )*
    ) => {
        $(
            fn $fn_name(&mut self, node: $node$( <$param> )?) -> $node$( <$param> )? {
                node
            }
        )*
    }
}

trait Visitor {
    generate_visit_fn! {
        enter_ident: Ident
        enter_body: Body
        enter_function_decl: DeclFunction
        enter_stmt_expr: StmtExpr
        enter_binary_expr: ExprBinary
        enter_stmt_list: StatementList<Stmt>
        enter_binding_element: BindingElement
        enter_format_parameters: FormalParameters
        enter_program: Program
        enter_expr: Expr
        enter_stmt: Stmt
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
    enum Program(enter: enter_program) {
        Script
        Module
    }

    enum Expr(enter: enter_expr) {
        Binary
        IdentRef
    }

    enum Stmt(enter: enter_stmt) {
        FunctionDecl
        Expr
    }
}

generate_fold! {
    Body(enter: enter_body) {
        statements
    }

    DeclFunction(enter: enter_function_decl) {
        identifier
        parameters
        body
    }

    StmtExpr(enter: enter_stmt_expr) {
        expr
    }

    ExprBinary(enter: enter_binary_expr) {
        left
        right
    }

    StatementList<Stmt>(enter: enter_stmt_list) {
        body
    }

    BindingElement(enter: enter_binding_element) { }

    FormalParameters(enter: enter_format_parameters) {
        bindings
    }

    Ident (enter: enter_ident) {}
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
        fn enter_ident(&mut self, mut ident: Ident) -> Ident {
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
