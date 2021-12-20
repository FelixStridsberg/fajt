use fajt_ast::traverse::{Traverse, Visitor};
use fajt_ast::*;

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
        self.push('}');
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
    fn enter_block_stmt(&mut self, node: &mut StmtBlock) -> bool {
        if node.statements.is_empty() {
            self.push_str("{}");
            return false;
        }

        self.block_start();
        self.indent();

        node.statements.traverse(self);

        self.dedent();
        self.block_end();
        false
    }

    fn enter_try_stmt(&mut self, node: &mut StmtTry) -> bool {
        self.push_str("try");
        self.space();
        node.block.traverse(self);

        if let Some(catch) = node.handler.as_mut() {
            self.space();
            self.push_str("catch");
            self.space();
            catch.traverse(self);
        }

        if let Some(finalizer) = node.finalizer.as_mut() {
            self.space();
            self.push_str("finally");
            self.space();
            finalizer.traverse(self);
        }

        false
    }

    fn enter_catch_clause(&mut self, node: &mut CatchClause) -> bool {
        if node.parameter.is_some() {
            self.push('(');
            node.parameter.traverse(self);
            self.push(')');
            self.space();
        }

        node.body.traverse(self);
        false
    }

    fn enter_labeled_stmt(&mut self, node: &mut StmtLabeled) -> bool {
        node.label.traverse(self);
        self.push(':');
        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_throw_stmt(&mut self, node: &mut StmtThrow) -> bool {
        self.push_str("throw");

        if node.argument.is_some() {
            self.push(' ');
            node.argument.traverse(self);
        }

        false
    }

    fn enter_switch_stmt(&mut self, node: &mut StmtSwitch) -> bool {
        self.push_str("switch");
        self.space();

        self.push('(');
        node.discriminant.traverse(self);
        self.push(')');

        self.space();

        if node.cases.is_empty() {
            self.push_str("{}");
            return false;
        }

        self.block_start();
        self.indent();

        node.cases.traverse(self);

        self.dedent();
        self.block_end();

        false
    }

    fn enter_if_stmt(&mut self, node: &mut StmtIf) -> bool {
        self.push_str("if");

        self.space();
        self.push('(');
        node.condition.traverse(self);
        self.push(')');
        self.new_line();

        self.indent();
        node.consequent.traverse(self);
        self.dedent();

        if node.alternate.is_some() {
            self.push_str("else");
            self.new_line();

            self.indent();
            node.alternate.traverse(self);
            self.dedent();
        }

        false
    }

    fn enter_for_stmt(&mut self, node: &mut StmtFor) -> bool {
        self.push_str("for");
        self.space();
        self.push('(');

        node.init.traverse(self);
        if node.init.is_none() || matches!(node.init, Some(ForInit::Expr(_))) {
            self.push(';');
        }

        node.test.traverse(self);
        self.push(';');

        node.update.traverse(self);
        self.push(')');

        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_for_of_stmt(&mut self, node: &mut StmtForOf) -> bool {
        self.push_str("for");

        if node.asynchronous {
            self.push_str(" await");
        }

        self.space();
        self.push('(');
        node.left.traverse(self);
        self.push_str(" of ");
        node.right.traverse(self);
        self.push(')');

        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_for_in_stmt(&mut self, node: &mut StmtForIn) -> bool {
        self.push_str("for");
        self.space();
        self.push('(');
        node.left.traverse(self);
        self.push_str(" in ");
        node.right.traverse(self);
        self.push(')');

        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_while_stmt(&mut self, node: &mut StmtWhile) -> bool {
        self.push_str("while");
        self.space();
        self.push('(');
        node.test.traverse(self);
        self.push(')');
        self.space();
        node.body.traverse(self);

        false
    }

    fn enter_do_while_stmt(&mut self, node: &mut StmtDoWhile) -> bool {
        self.push_str("do ");
        node.body.traverse(self);

        self.push_str("while");
        self.space();

        self.push('(');
        node.test.traverse(self);
        self.push(')');

        false
    }

    fn enter_switch_case(&mut self, node: &mut SwitchCase) -> bool {
        if node.test.is_some() {
            self.push_str("case ");
            node.test.traverse(self);
        } else {
            self.push_str("default");
        }

        self.push(':');
        self.new_line();

        self.indent();
        node.consequent.traverse(self);
        self.dedent();
        false
    }

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

        self.push_str("function");

        if node.generator {
            self.push('*');
        }

        self.push(' ');

        node.identifier.traverse(self);
        node.parameters.traverse(self);
        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_class_decl(&mut self, node: &mut DeclClass) -> bool {
        self.push_str("class ");

        node.identifier.traverse(self);

        if let Some(super_class) = node.super_class.as_mut() {
            self.push_str(" extends ");
            super_class.traverse(self);
        }

        self.space();

        if node.body.is_empty() {
            self.push_str("{}");
            return false;
        }

        self.block_start();
        self.indent();

        node.body.traverse(self);

        self.dedent();
        self.block_end();
        false
    }

    fn enter_class_method(&mut self, node: &mut ClassMethod) -> bool {
        if node.asynchronous {
            self.push_str("async ");
        }

        if node.generator {
            self.push('*');
        }

        match node.kind {
            ClassMethodKind::Method => {}
            ClassMethodKind::Get => {
                self.push_str("get ");
            }
            ClassMethodKind::Set => {
                self.push_str("set ");
            }
        }

        node.name.traverse(self);
        node.parameters.traverse(self);

        self.space();
        node.body.traverse(self);
        self.new_line();
        false
    }

    fn enter_body(&mut self, node: &mut Body) -> bool {
        if node.statements.is_empty() && node.directives.is_empty() {
            self.push_str("{}");
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
        self.push_str("return");

        if node.argument.is_some() {
            self.push(' ');
            node.argument.traverse(self);
        }

        self.push(';');
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

    fn enter_yield_expr(&mut self, node: &mut ExprYield) -> bool {
        self.push_str("yield");

        if node.delegate {
            self.push('*');
        }

        if node.argument.is_some() {
            self.push(' ');
            node.argument.traverse(self);
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

                if let Some(initializer) = initializer {
                    self.space();
                    self.push('=');
                    self.space();
                    initializer.traverse(self);
                }
            }
            ObjectBindingProp::KeyValue(name, prop) => {
                name.traverse(self);
                self.push(':');
                self.space();
                prop.traverse(self);
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

    fn enter_literal(&mut self, node: &mut Literal) -> bool {
        match node {
            Literal::Null => {
                self.push_str("null");
            }
            Literal::Boolean(true) => {
                self.push_str("true");
            }
            Literal::Boolean(false) => {
                self.push_str("false");
            }
            _ => {
                return true;
            }
        }

        false
    }

    fn enter_string_literal(&mut self, node: &mut LitString) -> bool {
        self.push(node.delimiter);
        self.push_str(&node.value);
        self.push(node.delimiter);
        false
    }

    fn enter_object_literal(&mut self, node: &mut Object) -> bool {
        if node.props.is_empty() {
            self.push_str("{}");
        } else {
            todo!()
        }
        false
    }

    fn enter_continue_stmt(&mut self, node: &mut StmtContinue) -> bool {
        self.push_str("continue");

        if node.label.is_some() {
            self.push(' ');
            node.label.traverse(self);
        }

        self.push(';');

        false
    }

    fn enter_break_stmt(&mut self, node: &mut StmtBreak) -> bool {
        self.push_str("break");

        if node.label.is_some() {
            self.push(' ');
            node.label.traverse(self);
        }

        self.push(';');
        false
    }

    fn exit_stmt(&mut self, _node: &mut Stmt) {
        if self.last_new_line != self.data.len() {
            self.new_line();
        }
    }

    fn enter_stmt_expr(&mut self, node: &mut StmtExpr) -> bool {
        node.expr.traverse(self);
        self.push(';');
        false
    }

    fn enter_with_stmt(&mut self, node: &mut StmtWith) -> bool {
        self.push_str("with(");
        node.object.traverse(self);
        self.push(')');
        self.space();
        node.body.traverse(self);
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

    fn enter_debugger_stmt(&mut self, _node: &mut StmtDebugger) -> bool {
        self.push_str("debugger");
        false
    }
}
