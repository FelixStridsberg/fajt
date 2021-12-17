use fajt_ast::traverse::{Traverse, Visitor};
use fajt_ast::*;
use std::process::id;

const INDENTATION_SIZE: usize = 4;

pub fn generate_code(mut program: Program) -> String {
    let mut codegen = CodeGenerator::new();
    program.traverse(&mut codegen);
    codegen.into_string()
}

struct CodeGenerator {
    indent: usize,
    align: Option<usize>,
    data: String,
    last_new_line: usize,
    last_block_start: usize,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            indent: 0,
            align: None,
            data: String::new(),
            last_new_line: 0,
            last_block_start: 0,
        }
    }

    fn into_string(self) -> String {
        self.data
    }
}

impl CodeGenerator {
    fn indent(&mut self) -> &mut Self {
        self.indent += 1;
        self
    }

    fn dedent(&mut self) -> &mut Self {
        self.indent -= 1;
        self
    }

    fn space(&mut self) -> &mut Self {
        self.push(' ');
        self
    }

    fn block_start(&mut self) -> &mut Self {
        self.push('{').new_line();
        self.last_block_start = self.data.len();
        self
    }

    fn block_end(&mut self) -> &mut Self {
        self.push('}').new_line();
        self
    }

    fn push(&mut self, ch: char) -> &mut Self {
        self.maybe_indent();
        self.data.push(ch);
        self
    }

    fn push_str(&mut self, str: &str) -> &mut Self {
        self.maybe_indent();
        self.data.push_str(str);
        self
    }

    fn start_align(&mut self) {
        self.align = Some(self.data.len() - self.last_new_line);
    }

    fn stop_align(&mut self) {
        self.align = None;
    }

    // Separation between logical sections, only adds a new line if not first in block or file.
    fn separation(&mut self) -> &mut Self {
        if self.data.len() != self.last_block_start {
            self.new_line();
        }
        self
    }

    fn new_line(&mut self) -> &mut Self {
        self.push('\n');
        self.last_new_line = self.data.len();
        self
    }

    fn maybe_indent(&mut self) {
        if !self.should_indent() {
            return;
        }

        if let Some(align) = self.align {
            self.data.push_str(&" ".repeat(align));
        } else {
            self.data
                .push_str(&" ".repeat(self.indent * INDENTATION_SIZE));
        }
    }

    fn should_indent(&self) -> bool {
        !self.data.is_empty() && self.data.len() == self.last_new_line
    }
}

impl Visitor for CodeGenerator {
    fn enter_binary_expr(&mut self, node: &mut ExprBinary) -> bool {
        node.left.traverse(self);
        self.space();
        self.push_str(&node.operator.to_string());
        self.space();
        node.right.traverse(self);
        false
    }

    fn enter_parenthesized_expr(&mut self, node: &mut ExprParenthesized) -> bool {
        self.push('(');
        node.expression.traverse(self);
        self.push(')');
        false
    }

    fn enter_function_decl(&mut self, node: &mut DeclFunction) -> bool {
        self.separation();

        if node.asynchronous {
            self.push_str("async ");
        }

        self.push_str("function ");

        if node.generator {
            self.push('*');
        }

        node.identifier.traverse(self);
        node.parameters.traverse(self);
        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_body(&mut self, node: &mut Body) -> bool {
        if node.statements.is_empty() && node.directives.is_empty() {
            self.push_str("{}").new_line();
            return false;
        }

        self.block_start();
        self.indent();

        for x in &mut node.directives {
            x.traverse(self);
            self.push(';');
            self.new_line();
        }

        node.statements.traverse(self);
        self.dedent();
        self.block_end();
        false
    }

    fn enter_return_stmt(&mut self, node: &mut StmtReturn) -> bool {
        self.push_str("return ");
        node.argument.traverse(self);
        self.push(';');
        self.new_line();
        false
    }

    fn enter_formal_parameters(&mut self, node: &mut FormalParameters) -> bool {
        self.push('(');

        let mut bindings = node.bindings.iter_mut().peekable();
        while let Some(bind) = bindings.next() {
            bind.traverse(self);
            if bindings.peek().is_some() {
                self.push(',');
                self.space();
            }
        }

        if let Some(rest) = node.rest.as_mut() {
            self.push_str("...");
            rest.traverse(self);
        }

        self.push(')');
        false
    }

    fn enter_empty_statement(&mut self, _node: &mut StmtEmpty) -> bool {
        self.push(';');
        self.new_line();
        false
    }

    fn enter_binding_element(&mut self, node: &mut BindingElement) -> bool {
        node.pattern.traverse(self);

        if let Some(initializer) = node.initializer.as_mut() {
            self.push_str(" = ");
            initializer.traverse(self);
        }
        false
    }

    fn enter_variable_stmt(&mut self, node: &mut StmtVariable) -> bool {
        let kind_str = node.kind.to_string();
        self.push_str(&kind_str);
        self.push(' ');

        self.start_align();

        let mut declarations = node.declarations.iter_mut().peekable();
        while let Some(decl) = declarations.next() {
            decl.traverse(self);

            if declarations.peek().is_some() {
                self.push(',');
                if decl.initializer.is_some() {
                    self.new_line();
                } else {
                    self.push(' ');
                }
            } else {
                self.push(';');
                self.new_line();
            }
        }

        self.stop_align();

        false
    }

    fn enter_array_binding(&mut self, node: &mut ArrayBinding) -> bool {
        if node.rest.is_none() && node.elements.is_empty() {
            self.push_str("[]");
            return false;
        }

        self.push('[');
        self.space();

        let mut elements = node.elements.iter_mut().peekable();
        while let Some(element) = elements.next() {
            element.traverse(self);

            if elements.peek().is_some() || element.is_none() {
                self.push(',');
            }

            if elements.peek().is_some() {
                self.space();
            }
        }

        if let Some(rest) = node.rest.as_mut() {
            if !node.elements.is_empty() {
                self.push(',');
                self.space();
            }

            self.push_str("...");
            rest.traverse(self);
        }

        self.space();
        self.push(']');

        false
    }

    fn enter_object_binding(&mut self, node: &mut ObjectBinding) -> bool {
        if node.rest.is_none() && node.props.is_empty() {
            self.push_str("{}");
            return false;
        }

        self.push('{');
        self.space();

        let mut props = node.props.iter_mut().peekable();
        while let Some(element) = props.next() {
            element.traverse(self);

            if props.peek().is_some() {
                self.push(',');
                self.space();
            }
        }

        if let Some(rest) = node.rest.as_mut() {
            if !node.props.is_empty() {
                self.push(',');
                self.space();
            }

            self.push_str("...");
            rest.traverse(self);
        }

        self.space();
        self.push('}');
        false
    }

    fn enter_object_binding_prop(&mut self, node: &mut ObjectBindingProp) -> bool {
        match node {
            ObjectBindingProp::Single(ident, initializer) => {
                ident.traverse(self);
            }
            ObjectBindingProp::KeyValue(name, prop) => {
                todo!()
            }
        }
        false
    }

    fn enter_variable_declaration(&mut self, node: &mut VariableDeclaration) -> bool {
        node.pattern.traverse(self);

        if let Some(initializer) = node.initializer.as_mut() {
            self.push_str(" = ");
            initializer.traverse(self);
        }

        false
    }

    fn enter_string_literal(&mut self, node: &mut LitString) -> bool {
        self.push(node.delimiter);
        self.push_str(&node.value);
        self.push(node.delimiter);
        false
    }

    fn enter_stmt_expr(&mut self, node: &mut StmtExpr) -> bool {
        node.expr.traverse(self);
        self.push(';');
        self.new_line();
        false
    }

    fn enter_number(&mut self, node: &mut Number) -> bool {
        match node {
            Number::Integer(n, _) => {
                self.push_str(&n.to_string());
            }
            Number::Decimal(_) => {}
        }

        false
    }

    fn enter_ident(&mut self, node: &mut Ident) -> bool {
        self.push_str(&node.name);
        false
    }
}
