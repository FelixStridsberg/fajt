#[macro_use]
mod macros;

use super::*;

pub trait Traverse {
    fn traverse(&mut self, visitor: &mut dyn Visitor);
}

impl<F: Traverse> Traverse for Vec<F> {
    fn traverse(&mut self, visitor: &mut dyn Visitor) {
        self.into_iter()
            .map(|node| node.traverse(visitor))
            .collect()
    }
}

impl<F: Traverse> Traverse for Box<F> {
    fn traverse(&mut self, visitor: &mut dyn Visitor) {
        (*self).as_mut().traverse(visitor)
    }
}

impl<F: Traverse> Traverse for Option<F> {
    fn traverse(&mut self, visitor: &mut dyn Visitor) {
        for node in self {
            node.traverse(visitor);
        }
    }
}

// This macro generates all traverse implementations and a Visitor trait with all methods required for
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
            Literal
            Parenthesized
        }

        Stmt: (enter: enter_stmt, exit: exit_stmt) {
            FunctionDecl
            Return
            Expr
        }

        BindingPattern: (enter: enter_binding_pattern, exit: exit_binding_pattern) {
            Ident
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

        ExprParenthesized: (enter: enter_parenthesized_expr, exit: exit_parenthesized_expr) {
            expression
        }

        ExprLiteral: (enter: enter_literal_expr, exit: exit_literal_expr) { }

        StatementList<Stmt>: (enter: enter_stmt_list, exit: exit_stmt_list) {
            body
        }

        BindingElement: (enter: enter_binding_element, exit: exit_binding_element) {
            pattern
        }

        FormalParameters: (enter: enter_formal_parameters, exit: exit_formal_parameters) {
            bindings
        }

        Ident: (enter: enter_ident, exit: exit_ident) {}

        StmtReturn: (enter: enter_return_stmt, exit: exit_return_stmt) {
            argument
        }
    }
}
