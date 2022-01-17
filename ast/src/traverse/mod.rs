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

// This macro generates all traverse implementations and a Visitor trait with all methods required
// for visiting all types defined here. Only the types and fields defined here are traversed.
// It also generates a TraceVisitor struct that can be used to trace the traversal of a tree.
generate_fold_and_visit! {
    enums: {
        Program: (enter: enter_program, exit: exit_program) {
            Script
            Module
        }

        Expr: (enter: enter_expr, exit: exit_expr) {
            ArrowFunction
            Assignment
            Await
            Binary
            Call
            Class
            Conditional
            Function
            IdentRef
            Literal
            Logical
            Member
            MetaProperty
            New
            OptionalCall
            OptionalMember
            Parenthesized
            Sequence
            This
            Unary
            Update
            Yield
        }

        Stmt: (enter: enter_stmt, exit: exit_stmt) {
            Block
            Break
            Continue
            Debugger
            DoWhile
            Empty
            Expr
            For
            ForIn
            ForOf
            If
            Labeled
            Return
            Switch
            Throw
            Try
            Variable
            While
            With
            ClassDecl
            FunctionDecl
            ImportDecl
            ExportDecl
        }

        BindingPattern: (enter: enter_binding_pattern, exit: exit_binding_pattern) {
            Ident
            Array
            Object
        }

        Literal: (enter: enter_literal, exit: exit_literal) {
            Number
            String
            Array
            Object
            Template
        }

        LitNumber: (enter: enter_number_literal, exit: exit_number_literal) {}

        TemplatePart: (enter: enter_template_part, exit: exit_template_part) {
            Expr
        }

        ObjectBindingProp: (enter: enter_object_binding_prop, exit: exit_object_binding_prop) {
            Single
            Named
        }

        PropertyName: (enter: enter_property_name, exit: exit_property_name) {
            Ident
            String
            Number
            Computed
        }

        ClassElement: (enter: enter_class_element, exit: exit_class_element) {
            Method
        }

        ForInit: (enter: enter_for_init, exit: exit_for_init) {
            Expr
            Declaration
        }

        Callee: (enter: enter_callee, exit: exit_callee) {
            Expr
        }

        Argument: (enter: enter_argument, exit: exit_argument) {
            Expr
            Spread
        }

        MemberObject: (enter: enter_member_object, exit: exit_member_object) {
            Expr
            Super
        }

        Super: (enter: enter_super, exit: exit_super) { }

        MemberProperty: (enter: enter_member_property, exit: exit_member_property) {
            Ident
            Expr
        }

        ArrayElement: (enter: enter_array_element, exit: exit_array_element) {
            Expr
            Spread
        }

        ArrowFunctionBody: (enter: enter_arrow_function_body, exit: exit_arrow_function_body) {
            Expr
            Body
        }

        PropertyDefinition: (enter: enter_property_definition, exit: exit_property_definition) {
            IdentRef
            Spread
            Named
            Method
        }

        BinaryOperator: (enter: enter_binary_operator, exit: exit_binary_operator) { }

        LogicalOperator: (enter: enter_logical_operator, exit: exit_logical_operator) { }

        AssignmentOperator: (enter: enter_assignment_operator, exit: exit_assignment_operator) { }

        UnaryOperator: (enter: enter_unary_operator, exit: exit_unary_operator) { }

        UpdateOperator: (enter: enter_update_operator, exit: exit_update_operator) { }

        DeclExport: (enter: enter_export, exit: exit_export) {
            Decl
            DefaultExpr
            DefaultDecl
            Named
            Namespace
        }
    }

    // The order of the fields for structs reflects the order of traversal. The order should follow the
    // order the syntax appear in the source as close as possible.
    structs: {
        DeclFunction: (enter: enter_function_decl, exit: exit_function_decl) {
            identifier
            parameters
            body
        }

        DeclClass: (enter: enter_class_decl, exit: exit_class_decl) {
            identifier
            super_class
            body
        }

        DeclImport: (enter: enter_import, exit: exit_import) {
            default_binding
            named_imports
            named_imports
            from
        }

        StmtExpr: (enter: enter_stmt_expr, exit: exit_stmt_expr) {
            expr
        }

        StmtBlock: (enter: enter_block_stmt, exit: exit_block_stmt) {
            statements
        }

        ExprBinary: (enter: enter_binary_expr, exit: exit_binary_expr) {
            left
            operator
            right
        }

        ExprLogical: (enter: enter_logical_expr, exit: exit_logical_expr) {
            left
            operator
            right
        }

        ExprUnary: (enter: enter_unary_expr, exit: exit_unary_expr) {
            operator
            argument
        }

        ExprCall: (enter: enter_call_expr, exit: exit_call_expr) {
            callee
            arguments
        }

        ExprMember: (enter: enter_member_expr, exit: exit_member_expr) {
            object
            property
        }

        ExprOptionalMember: (enter: enter_optional_member_expr, exit: exit_optional_member_expr) {
            object
            property
        }

        ExprOptionalCall: (enter: enter_optional_call_expr, exit: exit_optional_call_expr) {
            callee
            arguments
        }

        ExprParenthesized: (enter: enter_parenthesized_expr, exit: exit_parenthesized_expr) {
            expression
        }

        ExprLiteral: (enter: enter_literal_expr, exit: exit_literal_expr) {
            literal
        }

        ExprAssignment: (enter: enter_assignment_expr, exit: exit_assignment_expr) {
            left
            operator
            right
        }

        ExprYield: (enter: enter_yield_expr, exit: exit_yield_expr) {
            argument
        }

        ExprConditional: (enter: enter_conditional_expr, exit: exit_conditional_expr) {
            condition
            consequent
            condition
        }

        ExprUpdate: (enter: enter_update_expr, exit: exit_update_expr) {
            argument
            operator
        }

        ExprSequence: (enter: enter_sequence_expr, exit: exit_sequence_expr) {
            expr
        }

        ExprThis: (enter: enter_this_expr, exit: exit_this_expr) {}

        ExprFunction: (enter: enter_function_expr, exit: exit_function_expr) {
            identifier
            parameters
            body
        }

        ExprNew: (enter: enter_new_expr, exit: exit_new_expr) {
            callee
            arguments
        }

        ExprArrowFunction: (enter: enter_arrow_function, exit: exit_arrow_function) {
            parameters
            body
        }

        ExprAwait: (enter: enter_await_expr, exit: exit_await_expr) {
            argument
        }

        ExprMetaProperty: (enter: enter_meta_property, exit: exit_meta_property) {
            meta
            property
        }

        ExprClass: (enter: enter_class_expr, exit: exit_class_expr) {
            identifier
            super_class
            body
        }

        StmtReturn: (enter: enter_return_stmt, exit: exit_return_stmt) {
            argument
        }

        StmtVariable: (enter: enter_variable_stmt, exit: exit_variable_stmt) {
            declarations
        }

        StmtEmpty: (enter: enter_empty_statement, exit: exit_empty_statement) {}

        StmtTry: (enter: enter_try_stmt, exit: exit_try_stmt) {
            block
            handler
            finalizer
        }

        StmtDebugger: (enter: enter_debugger_stmt, exit: exit_debugger_stmt) {}

        StmtThrow: (enter: enter_throw_stmt, exit: exit_throw_stmt) {
            argument
        }

        StmtLabeled: (enter: enter_labeled_stmt, exit: exit_labeled_stmt) {
            label
            body
        }

        StmtWith: (enter: enter_with_stmt, exit: exit_with_stmt) {
            object
            body
        }

        StmtContinue: (enter: enter_continue_stmt, exit: exit_continue_stmt) {
            label
        }

        StmtBreak: (enter: enter_break_stmt, exit: exit_break_stmt) {
            label
        }

        StmtSwitch: (enter: enter_switch_stmt, exit: exit_switch_stmt) {
            discriminant
            cases
        }

        StmtIf: (enter: enter_if_stmt, exit: exit_if_stmt) {
            condition
            consequent
            alternate
        }

        StmtFor: (enter: enter_for_stmt, exit: exit_for_stmt) {
            init
            test
            update
            body
        }

        StmtForOf: (enter: enter_for_of_stmt, exit: exit_for_of_stmt) {
            left
            right
            body
        }

        StmtForIn: (enter: enter_for_in_stmt, exit: exit_for_in_stmt) {
            left
            right
            body
        }

        StmtWhile: (enter: enter_while_stmt, exit: exit_while_stmt) {
            test
            body
        }

        StmtDoWhile: (enter: enter_do_while_stmt, exit: exit_do_while_stmt) {
            body
            test
        }

        StmtList<Stmt>: (enter: enter_stmt_list, exit: exit_stmt_list) {
            body
        }

        BindingElement: (enter: enter_binding_element, exit: exit_binding_element) {
            pattern
            initializer
        }

        FormalParameters: (enter: enter_formal_parameters, exit: exit_formal_parameters) {
            bindings
            rest
        }

        VariableDeclaration: (enter: enter_variable_declaration, exit: exit_variable_declaration) {
            pattern
            initializer
        }

        LitString: (enter: enter_string_literal, exit: exit_string_literal) {}

        LitArray: (enter: enter_array_literal, exit: exit_array_literal) {
            elements
        }

        LitObject: (enter: enter_object_literal, exit: exit_object_literal) {
            props
        }

        ArrayBinding: (enter: enter_array_binding, exit: exit_array_binding) {
            elements
            rest
        }

        ObjectBinding: (enter: enter_object_binding, exit: exit_object_binding) {
            props
            rest
        }

        SingleNameBinding: (enter: enter_single_name_binding, exit: exit_single_name_binding) {
            ident
            initializer
        }

        NamedBinding: (enter: enter_named_binding, exit: exit_named_binding) {
            property
            binding
        }

        NamedProperty: (enter: enter_named_property, exit: exit_named_property) {
            name
            value
        }

        MethodDefinition: (enter: enter_method_definition, exit: exit_method_definition) {
            name
            parameters
            body
        }

        CatchClause: (enter: enter_catch_clause, exit: exit_catch_clause) {
            parameter
            body
        }

        SwitchCase: (enter: enter_switch_case, exit: exit_switch_case) {
            test
            consequent
        }

        ExportDecl: (enter: enter_export_decl, exit: exit_export_decl) {
            decl
        }

        ExportDefaultDecl: (enter: enter_export_default_decl, exit: exit_export_default_decl) {
            decl
        }

        ExportDefaultExpr: (enter: enter_export_default_expr, exit: exit_export_default_expr) {
            expr
        }

        ExportNamed: (enter: enter_export_named, exit: exit_export_named) {
            named_exports
            from
        }

        ExportNamespace: (enter: enter_export_namespace, exit: exit_export_namespace) {
            alias
            from
        }

        NamedExport: (enter: enter_named_export, exit: exit_named_export) {
            name
            alias_of
        }

        NamedImport: (enter: enter_named_import, exit: exit_named_import) {
            name
            alias
        }

        Ident: (enter: enter_ident, exit: exit_ident) {}

        Body: (enter: enter_body, exit: exit_body) {
            directives
            statements
        }
    }
}
