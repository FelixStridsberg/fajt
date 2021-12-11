#[macro_use]
mod fold;

use fajt_ast::*;

macro_rules! generate_visit_trait {
    (
        $( $fn_name:ident: $node:ident $( <$param:ident> )?  )*
    ) => {
        pub trait Visitor {
            $(
                fn $fn_name(&mut self, node: $node$( <$param> )?) -> $node$( <$param> )? {
                    node
                }
            )*
        }

        pub struct TraceVisitor {
            pub visits: Vec<&'static str>,
        }

        impl Visitor for TraceVisitor {
            $(
                fn $fn_name(&mut self, node: $node$( <$param> )?) -> $node$( <$param> )? {
                    self.visits.push(stringify!($fn_name));
                    node
                }
            )*
        }
    }
}

generate_visit_trait! {
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
    enter_return_stmt: StmtReturn

    exit_ident: Ident
    exit_body: Body
    exit_function_decl: DeclFunction
    exit_stmt_expr: StmtExpr
    exit_binary_expr: ExprBinary
    exit_stmt_list: StatementList<Stmt>
    exit_binding_element: BindingElement
    exit_format_parameters: FormalParameters
    exit_program: Program
    exit_expr: Expr
    exit_stmt: Stmt
    exit_return_stmt: StmtReturn
}

pub trait Fold {
    fn fold(self, visitor: &mut dyn Visitor) -> Self;
}

impl<F: Fold> Fold for Vec<F> {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        self.into_iter().map(|node| node.fold(visitor)).collect()
    }
}

impl<F: Fold> Fold for Box<F> {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        Box::new((*self).fold(visitor))
    }
}

impl<F: Fold> Fold for Option<F> {
    fn fold(self, visitor: &mut dyn Visitor) -> Self {
        self.map(|node| node.fold(visitor))
    }
}

generate_fold! {
    enum Program(enter: enter_program, exit: exit_program) {
        Script
        Module
    }

    enum Expr(enter: enter_expr, exit: exit_expr) {
        Binary
        IdentRef
    }

    enum Stmt(enter: enter_stmt, exit: exit_stmt) {
        FunctionDecl
        Return
        Expr
    }
}

generate_fold! {
    Body(enter: enter_body, exit: exit_body) {
        statements
    }

    DeclFunction(enter: enter_function_decl, exit: exit_function_decl) {
        identifier
        parameters
        body
    }

    StmtExpr(enter: enter_stmt_expr, exit: exit_stmt_expr) {
        expr
    }

    ExprBinary(enter: enter_binary_expr, exit: exit_binary_expr) {
        left
        right
    }

    StatementList<Stmt>(enter: enter_stmt_list, exit: exit_stmt_list) {
        body
    }

    BindingElement(enter: enter_binding_element, exit: exit_binding_element) { }

    FormalParameters(enter: enter_format_parameters, exit: exit_format_parameters) {
        bindings
    }

    Ident (enter: enter_ident, exit: exit_ident) {}

    StmtReturn (enter: enter_return_stmt, exit: exit_return_stmt) {
        argument
    }
}
