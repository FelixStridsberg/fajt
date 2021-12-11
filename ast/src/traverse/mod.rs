#[macro_use]
mod macros;

use super::*;

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

// This macro generates all fold implementations and a Visitor trait with all methods required for
// visiting all types defined here. Only the types and fields defined here are traversed.
// It also generates a TraceVisitor struct that can be used to trace the traversal of a tree.
generate_fold_and_visit! {
    enums: {
        Program: (enter: enter_program, exit: exit_program) {
            Script
            Module
        }

        Expr: (enter: enter_expr, exit: exit_expr) {
            Binary
            IdentRef
        }

        Stmt: (enter: enter_stmt, exit: exit_stmt) {
            FunctionDecl
            Return
            Expr
        }
    }

    structs: {
        Body: (enter: enter_body, exit: exit_body) {
            statements
        }

        DeclFunction: (enter: enter_function_decl, exit: exit_function_decl) {
            identifier
            parameters
            body
        }

        StmtExpr: (enter: enter_stmt_expr, exit: exit_stmt_expr) {
            expr
        }

        ExprBinary: (enter: enter_binary_expr, exit: exit_binary_expr) {
            left
            right
        }

        StatementList<Stmt>: (enter: enter_stmt_list, exit: exit_stmt_list) {
            body
        }

        BindingElement: (enter: enter_binding_element, exit: exit_binding_element) { }

        FormalParameters: (enter: enter_format_parameters, exit: exit_format_parameters) {
            bindings
        }

        Ident: (enter: enter_ident, exit: exit_ident) {}

        StmtReturn: (enter: enter_return_stmt, exit: exit_return_stmt) {
            argument
        }
    }
}
