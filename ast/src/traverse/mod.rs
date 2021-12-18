#[macro_use]
mod macros;

use super::*;

pub trait Traverse {
    fn traverse(&mut self, visitor: &mut dyn Visitor);
}

impl<F: Traverse> Traverse for Vec<F> {
    fn traverse(&mut self, visitor: &mut dyn Visitor) {
        self.iter_mut().map(|node| node.traverse(visitor)).collect()
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
            Yield
        }

        Stmt: (enter: enter_stmt, exit: exit_stmt) {
            FunctionDecl
            Return
            Expr
            Variable
            Empty
            Block
            Class
            Try
            Debugger
            Throw
            Labeled
            With
            Continue
        }

        BindingPattern: (enter: enter_binding_pattern, exit: exit_binding_pattern) {
            Ident
            Array
            Object
        }

        Literal: (enter: enter_literal, exit: exit_literal) {
            Number
            String
            Object
        }

        Number: (enter: enter_number, exit: exit_number) {}

        ObjectBindingProp: (enter: enter_object_binding_prop, exit: exit_object_binding_prop) {}

        PropertyName: (enter: enter_property_name, exit: exit_property_name) {
            Ident
        }

        ClassElement: (enter: enter_class_element, exit: exit_class_element) {
            Method
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

        StmtBlock: (enter: enter_block_stmt, exit: exit_block_stmt) { }

        ExprBinary: (enter: enter_binary_expr, exit: exit_binary_expr) {
            left
            right
        }

        ExprParenthesized: (enter: enter_parenthesized_expr, exit: exit_parenthesized_expr) {
            expression
        }

        ExprLiteral: (enter: enter_literal_expr, exit: exit_literal_expr) {
            literal
        }

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

        StmtVariable: (enter: enter_variable_stmt, exit: exit_variable_stmt) {
            declarations
        }

        VariableDeclaration: (enter: enter_variable_declaration, exit: exit_variable_declaration) {
            pattern
            initializer
        }

        StmtEmpty: (enter: enter_empty_statement, exit: exit_empty_statement) {}

        LitString: (enter: enter_string_literal, exit: exit_string_literal) {}

        Object: (enter: enter_object_literal, exit: exit_object_literal) {}

        ArrayBinding: (enter: enter_array_binding, exit: exit_array_binding) {}

        ObjectBinding: (enter: enter_object_binding, exit: exit_object_binding) {}

        ExprYield: (enter: enter_yield_expr, exit: exit_yield_expr) {
            argument
        }

        DeclClass: (enter: enter_class_decl, exit: exit_class_decl) {
            super_class
            identifier
            body
        }

        ClassMethod: (enter: enter_class_method, exit: exit_class_method) {
            name
            parameters
            body
        }


        StmtTry: (enter: enter_try_stmt, exit: exit_try_stmt) {
            handler
            block
        }

        CatchClause: (enter: enter_catch_clause, exit: exit_catch_clause) {
            parameter
            body
        }

        StmtDebugger: (enter: enter_debugger_stmt, exit: exit_debugger_stmt) {}

        StmtThrow: (enter: enter_throw_stmt, exit: exit_throw_stmt) {}

        StmtLabeled: (enter: enter_labeled_stmt, exit: exit_labeled_stmt) {
            label
            body
        }

        StmtWith: (enter: enter_with_stmt, exit: exit_with_stmt) {}

        StmtContinue: (enter: enter_continue_stmt, exit: exit_continue_stmt) {}
    }
}
