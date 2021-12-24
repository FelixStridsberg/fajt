use fajt_ast::traverse::{Traverse, Visitor};
use fajt_ast::*;
use std::cell::RefCell;
use std::rc::Rc;

const INDENTATION_SIZE: usize = 4;

pub fn generate_code(mut program: Program) -> String {
    let mut data = String::new();
    let mut codegen = CodeGenerator::new(&mut data);
    program.traverse(&mut codegen);
    data
}

struct GeneratorContext {
    indent: usize,
    align: Option<usize>,
}

impl GeneratorContext {
    fn new() -> Self {
        GeneratorContext {
            indent: 0,
            align: None,
        }
    }
}

struct Positions {
    last_new_line: usize,
    last_block_start: usize,
}

struct CodeGenerator<'a> {
    data: &'a mut String,
    ctx: GeneratorContext,
    pos: Rc<RefCell<Positions>>,
}

impl<'a> CodeGenerator<'a> {
    fn new(data: &'a mut String) -> Self {
        CodeGenerator {
            data,
            ctx: GeneratorContext::new(),
            pos: Rc::new(RefCell::new(Positions {
                last_new_line: 0,
                last_block_start: 0,
            })),
        }
    }

    fn last_new_line(&self) -> usize {
        (*self.pos).borrow().last_new_line
    }

    fn set_new_line(&mut self, pos: usize) {
        self.pos.borrow_mut().last_new_line = pos;
    }

    fn last_block_start(&self) -> usize {
        (*self.pos).borrow().last_block_start
    }

    fn set_block_start(&mut self, pos: usize) {
        self.pos.borrow_mut().last_block_start = pos;
    }
}

impl CodeGenerator<'_> {
    /// Current byte position from start.
    fn pos(&self) -> usize {
        self.data.len()
    }

    /// Current byte position from start of current line.
    fn col_pos(&self) -> usize {
        self.pos() - self.last_new_line()
    }

    fn with_indent(&mut self) -> CodeGenerator<'_> {
        CodeGenerator {
            data: self.data,
            pos: self.pos.clone(),
            ctx: GeneratorContext {
                indent: self.ctx.indent + 1,
                ..self.ctx
            },
        }
    }

    fn with_align(&mut self) -> CodeGenerator<'_> {
        let align = self.col_pos();
        CodeGenerator {
            data: self.data,
            pos: self.pos.clone(),
            ctx: GeneratorContext {
                align: Some(align),
                ..self.ctx
            },
        }
    }

    fn space(&mut self) -> &mut Self {
        self.push(' ');
        self
    }

    fn block_start(&mut self) -> &mut Self {
        self.push('{').new_line();
        self.set_block_start(self.pos());
        self
    }

    fn at_block_start(&self) -> bool {
        self.last_block_start() == self.pos()
    }

    fn block_end(&mut self) -> &mut Self {
        if self.at_block_start() {
            self.data.pop(); // Pop \n from empty blocks
            self.set_new_line(0); // TODO make better
        }

        self.push('}');
        self
    }

    fn push(&mut self, ch: char) -> &mut Self {
        self.maybe_indent();
        self.data.push(ch);
        self
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn push_str(&mut self, str: &str) -> &mut Self {
        self.maybe_indent();
        self.data.push_str(str);
        self
    }

    // Separation between logical sections, only adds a new line if not first in block or file.
    fn separation(&mut self) -> &mut Self {
        if !self.at_block_start() {
            self.new_line();
        }
        self
    }

    fn new_line(&mut self) -> &mut Self {
        self.push('\n');
        self.set_new_line(self.pos());
        self
    }

    fn maybe_indent(&mut self) {
        if !self.should_indent() {
            return;
        }

        if let Some(align) = self.ctx.align {
            self.data.push_str(&" ".repeat(align));
        } else {
            self.data
                .push_str(&" ".repeat(self.ctx.indent * INDENTATION_SIZE));
        }
    }

    fn should_indent(&self) -> bool {
        !self.data.is_empty() && self.pos() == self.last_new_line()
    }
}

impl Visitor for CodeGenerator<'_> {
    fn enter_block_stmt(&mut self, node: &mut StmtBlock) -> bool {
        self.block_start();
        node.statements.traverse(&mut self.with_indent());
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

        self.block_start();
        node.cases.traverse(&mut self.with_indent());
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

        node.consequent.traverse(&mut self.with_indent());

        if node.alternate.is_some() {
            self.push_str("else");
            self.new_line();

            node.alternate.traverse(&mut self.with_indent());
        }

        false
    }

    fn enter_for_stmt(&mut self, node: &mut StmtFor) -> bool {
        self.push_str("for");

        self.space();
        self.push('(');
        node.init.traverse(self);
        self.push(';');

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

    fn exit_for_init(&mut self, node: &mut ForInit) {
        if matches!(node, ForInit::Declaration(_)) {
            // Variable statements ends with semicolon, don't want that inside for syntax.
            self.pop();
        }
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

        node.consequent.traverse(&mut self.with_indent());
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

        self.block_start();
        node.body.traverse(&mut self.with_indent());
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
        self.block_start();

        let mut printer = self.with_indent();
        for x in &mut node.directives {
            x.traverse(&mut printer);
            printer.push(';');
            printer.new_line();
        }

        node.statements.traverse(&mut printer);
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

        let mut printer = self.with_align();
        let mut declarations = node.declarations.iter_mut().peekable();
        while let Some(decl) = declarations.next() {
            decl.traverse(&mut printer);

            if declarations.peek().is_some() {
                printer.push(',');
                if decl.initializer.is_some() {
                    printer.new_line();
                } else {
                    printer.push(' ');
                }
            } else {
                printer.push(';');
            }
        }

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

    fn exit_stmt_expr(&mut self, _node: &mut StmtExpr) {
        self.push(';');
    }

    fn exit_stmt(&mut self, _node: &mut Stmt) {
        if self.last_new_line() != self.pos() {
            self.new_line();
        }
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
