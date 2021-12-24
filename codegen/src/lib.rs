use fajt_ast::traverse::{Traverse, Visitor};
use fajt_ast::*;
use std::cell::Cell;
use std::rc::Rc;

pub fn generate_code(mut program: Program, ctx: GeneratorContext) -> String {
    let mut data = String::new();
    let mut codegen = CodeGenerator::new(&mut data, ctx);
    program.traverse(&mut codegen);
    data
}

#[derive(Clone)]
pub struct GeneratorContext {
    pub minified: bool,
    indent_size: usize,
    indent: usize,
    align: Option<usize>,
}

impl GeneratorContext {
    pub fn new() -> Self {
        GeneratorContext {
            minified: false,
            indent_size: 4,
            indent: 0,
            align: None,
        }
    }

    fn indentation(&self) -> usize {
        if let Some(align) = self.align {
            align
        } else {
            self.indent * self.indent_size
        }
    }
}

struct Index {
    last_new_line: Cell<usize>,
    last_block_start: Cell<usize>,
}

impl Index {
    fn new() -> Self {
        Index {
            last_new_line: Cell::new(0),
            last_block_start: Cell::new(0),
        }
    }

    fn last_new_line(&self) -> usize {
        self.last_new_line.get()
    }

    fn set_new_line(&self, pos: usize) {
        self.last_new_line.replace(pos);
    }

    fn last_block_start(&self) -> usize {
        self.last_block_start.get()
    }

    fn set_block_start(&self, pos: usize) {
        self.last_block_start.replace(pos);
    }
}

struct CodeGenerator<'a> {
    data: &'a mut String,
    ctx: GeneratorContext,
    index: Rc<Index>,
}

impl<'a> CodeGenerator<'a> {
    fn new(data: &'a mut String, ctx: GeneratorContext) -> Self {
        CodeGenerator {
            data,
            ctx,
            index: Rc::new(Index::new()),
        }
    }

    fn comma_separated_with_rest<I, R>(&mut self, items: &mut Vec<I>, rest: &mut Option<R>)
    where
        I: Traverse,
        R: Traverse,
    {
        let mut iter = items.iter_mut().peekable();
        while let Some(item) = iter.next() {
            item.traverse(self);
            if iter.peek().is_some() {
                self.char(',');
                self.space();
            }
        }

        if let Some(rest) = rest.as_mut() {
            if !items.is_empty() {
                self.char(',');
                self.space();
            }

            self.string("...");
            rest.traverse(self);
        }
    }
}

impl CodeGenerator<'_> {
    fn with_indent(&mut self) -> CodeGenerator<'_> {
        CodeGenerator {
            data: self.data,
            index: self.index.clone(),
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
            index: self.index.clone(),
            ctx: GeneratorContext {
                align: Some(align),
                ..self.ctx
            },
        }
    }

    /// Remove last char in output.
    fn remove_last(&mut self) {
        self.data.pop();
    }

    /// Current byte position from start.
    fn pos(&self) -> usize {
        self.data.len()
    }

    /// Current byte position from start of current line.
    fn col_pos(&self) -> usize {
        self.pos() - self.index.last_new_line.get()
    }

    /// Add char to output.
    fn char(&mut self, ch: char) {
        self.indent();
        self.data.push(ch);
    }

    /// Add string to output.
    fn string(&mut self, str: &str) {
        self.indent();
        self.data.push_str(str);
    }

    /// Adds formatting space.
    fn space(&mut self) {
        if !self.ctx.minified {
            self.char(' ');
        }
    }

    fn start_block(&mut self) {
        self.char('{');
        self.new_line();
        self.index.set_block_start(self.pos());
    }

    fn end_block(&mut self) {
        if self.at_block_start() {
            self.remove_last(); // Remove \n from empty blocks
            self.index.set_new_line(0); // Reset new line index since we are no longer at new line.
        }

        self.char('}');
    }

    fn at_block_start(&self) -> bool {
        self.index.last_block_start() == self.pos()
    }

    /// Separation between logical sections, only adds a new line if not first in block or file.
    fn separation(&mut self) {
        if !self.at_block_start() {
            self.new_line();
        }
    }

    fn new_line(&mut self) {
        self.char('\n');
        self.index.set_new_line(self.pos());
    }

    fn indent(&mut self) {
        if self.col_pos() != 0 {
            return;
        }

        self.data.push_str(&" ".repeat(self.ctx.indentation()));
    }
}

impl Visitor for CodeGenerator<'_> {
    fn enter_block_stmt(&mut self, node: &mut StmtBlock) -> bool {
        self.start_block();
        node.statements.traverse(&mut self.with_indent());
        self.end_block();
        false
    }

    fn enter_try_stmt(&mut self, node: &mut StmtTry) -> bool {
        self.string("try");
        self.space();
        node.block.traverse(self);

        if let Some(catch) = node.handler.as_mut() {
            self.space();
            self.string("catch");
            self.space();
            catch.traverse(self);
        }

        if let Some(finalizer) = node.finalizer.as_mut() {
            self.space();
            self.string("finally");
            self.space();
            finalizer.traverse(self);
        }

        false
    }

    fn enter_catch_clause(&mut self, node: &mut CatchClause) -> bool {
        if node.parameter.is_some() {
            self.char('(');
            node.parameter.traverse(self);
            self.char(')');
            self.space();
        }

        node.body.traverse(self);
        false
    }

    fn enter_labeled_stmt(&mut self, node: &mut StmtLabeled) -> bool {
        node.label.traverse(self);
        self.char(':');
        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_throw_stmt(&mut self, node: &mut StmtThrow) -> bool {
        self.string("throw");

        if node.argument.is_some() {
            self.char(' ');
            node.argument.traverse(self);
        }

        false
    }

    fn enter_switch_stmt(&mut self, node: &mut StmtSwitch) -> bool {
        self.string("switch");

        self.space();
        self.char('(');
        node.discriminant.traverse(self);
        self.char(')');
        self.space();

        self.start_block();
        node.cases.traverse(&mut self.with_indent());
        self.end_block();

        false
    }

    fn enter_if_stmt(&mut self, node: &mut StmtIf) -> bool {
        self.string("if");

        self.space();
        self.char('(');
        node.condition.traverse(self);
        self.char(')');
        self.new_line();

        node.consequent.traverse(&mut self.with_indent());

        if node.alternate.is_some() {
            self.string("else");
            self.new_line();

            node.alternate.traverse(&mut self.with_indent());
        }

        false
    }

    fn enter_for_stmt(&mut self, node: &mut StmtFor) -> bool {
        self.string("for");

        self.space();
        self.char('(');
        node.init.traverse(self);
        self.char(';');

        node.test.traverse(self);
        self.char(';');

        node.update.traverse(self);
        self.char(')');
        self.space();

        node.body.traverse(self);
        false
    }

    fn enter_for_of_stmt(&mut self, node: &mut StmtForOf) -> bool {
        self.string("for");

        if node.asynchronous {
            self.string(" await");
        }

        self.space();
        self.char('(');
        node.left.traverse(self);
        self.string(" of ");
        node.right.traverse(self);
        self.char(')');

        self.space();
        node.body.traverse(self);
        false
    }

    fn exit_for_init(&mut self, node: &mut ForInit) {
        if matches!(node, ForInit::Declaration(_)) {
            // Variable statements ends with semicolon, don't want that inside for syntax.
            self.remove_last();
        }
    }

    fn enter_for_in_stmt(&mut self, node: &mut StmtForIn) -> bool {
        self.string("for");
        self.space();
        self.char('(');
        node.left.traverse(self);
        self.string(" in ");
        node.right.traverse(self);
        self.char(')');

        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_while_stmt(&mut self, node: &mut StmtWhile) -> bool {
        self.string("while");
        self.space();
        self.char('(');
        node.test.traverse(self);
        self.char(')');
        self.space();
        node.body.traverse(self);

        false
    }

    fn enter_do_while_stmt(&mut self, node: &mut StmtDoWhile) -> bool {
        self.string("do ");
        node.body.traverse(self);

        self.string("while");
        self.space();

        self.char('(');
        node.test.traverse(self);
        self.char(')');

        false
    }

    fn enter_switch_case(&mut self, node: &mut SwitchCase) -> bool {
        if node.test.is_some() {
            self.string("case ");
            node.test.traverse(self);
        } else {
            self.string("default");
        }

        self.char(':');
        self.new_line();

        node.consequent.traverse(&mut self.with_indent());
        false
    }

    fn enter_binary_expr(&mut self, node: &mut ExprBinary) -> bool {
        node.left.traverse(self);
        self.space();
        self.string(&node.operator.to_string());
        self.space();
        node.right.traverse(self);
        false
    }

    fn enter_parenthesized_expr(&mut self, node: &mut ExprParenthesized) -> bool {
        self.char('(');
        node.expression.traverse(self);
        self.char(')');
        false
    }

    fn enter_function_decl(&mut self, node: &mut DeclFunction) -> bool {
        self.separation();

        if node.asynchronous {
            self.string("async ");
        }

        self.string("function");

        if node.generator {
            self.char('*');
        }

        self.char(' ');

        node.identifier.traverse(self);
        node.parameters.traverse(self);
        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_class_decl(&mut self, node: &mut DeclClass) -> bool {
        self.string("class ");

        node.identifier.traverse(self);

        if let Some(super_class) = node.super_class.as_mut() {
            self.string(" extends ");
            super_class.traverse(self);
        }

        self.space();

        self.start_block();
        node.body.traverse(&mut self.with_indent());
        self.end_block();
        false
    }

    fn enter_class_method(&mut self, node: &mut ClassMethod) -> bool {
        if node.asynchronous {
            self.string("async ");
        }

        if node.generator {
            self.char('*');
        }

        match node.kind {
            ClassMethodKind::Get => self.string("get "),
            ClassMethodKind::Set => self.string("set "),
            ClassMethodKind::Method => {}
        }

        node.name.traverse(self);
        node.parameters.traverse(self);

        self.space();
        node.body.traverse(self);
        self.new_line();
        false
    }

    fn enter_body(&mut self, node: &mut Body) -> bool {
        self.start_block();

        let mut printer = self.with_indent();
        for x in &mut node.directives {
            x.traverse(&mut printer);
            printer.char(';');
            printer.new_line();
        }

        node.statements.traverse(&mut printer);
        self.end_block();
        false
    }

    fn enter_return_stmt(&mut self, node: &mut StmtReturn) -> bool {
        self.string("return");

        if node.argument.is_some() {
            self.char(' ');
            node.argument.traverse(self);
        }

        self.char(';');
        false
    }

    fn enter_formal_parameters(&mut self, node: &mut FormalParameters) -> bool {
        self.char('(');
        self.comma_separated_with_rest(&mut node.bindings, &mut node.rest);
        self.char(')');
        false
    }

    fn enter_empty_statement(&mut self, _node: &mut StmtEmpty) -> bool {
        self.char(';');
        false
    }

    fn enter_binding_element(&mut self, node: &mut BindingElement) -> bool {
        node.pattern.traverse(self);

        if let Some(initializer) = node.initializer.as_mut() {
            self.string(" = ");
            initializer.traverse(self);
        }
        false
    }

    fn enter_yield_expr(&mut self, node: &mut ExprYield) -> bool {
        self.string("yield");

        if node.delegate {
            self.char('*');
        }

        if node.argument.is_some() {
            self.char(' ');
            node.argument.traverse(self);
        }

        false
    }

    fn enter_variable_stmt(&mut self, node: &mut StmtVariable) -> bool {
        let kind_str = node.kind.to_string();
        self.string(&kind_str);
        self.char(' ');

        let mut printer = self.with_align();
        let mut declarations = node.declarations.iter_mut().peekable();
        while let Some(decl) = declarations.next() {
            decl.traverse(&mut printer);

            if declarations.peek().is_some() {
                printer.char(',');
                if decl.initializer.is_some() {
                    printer.new_line();
                } else {
                    printer.char(' ');
                }
            } else {
                printer.char(';');
            }
        }

        false
    }

    fn enter_array_binding(&mut self, node: &mut ArrayBinding) -> bool {
        if node.rest.is_none() && node.elements.is_empty() {
            self.string("[]");
            return false;
        }

        self.char('[');
        self.space();

        let mut elements = node.elements.iter_mut().peekable();
        while let Some(element) = elements.next() {
            element.traverse(self);

            if elements.peek().is_some() || element.is_none() {
                self.char(',');
            }

            if elements.peek().is_some() {
                self.space();
            }
        }

        if let Some(rest) = node.rest.as_mut() {
            if !node.elements.is_empty() {
                self.char(',');
                self.space();
            }

            self.string("...");
            rest.traverse(self);
        }

        self.space();
        self.char(']');

        false
    }

    fn enter_object_binding(&mut self, node: &mut ObjectBinding) -> bool {
        if node.rest.is_none() && node.props.is_empty() {
            self.string("{}");
            return false;
        }

        self.char('{');
        self.space();
        self.comma_separated_with_rest(&mut node.props, &mut node.rest);
        self.space();
        self.char('}');
        false
    }

    fn enter_object_binding_prop(&mut self, node: &mut ObjectBindingProp) -> bool {
        match node {
            ObjectBindingProp::Single(ident, initializer) => {
                ident.traverse(self);

                if let Some(initializer) = initializer {
                    self.space();
                    self.char('=');
                    self.space();
                    initializer.traverse(self);
                }
            }
            ObjectBindingProp::KeyValue(name, prop) => {
                name.traverse(self);
                self.char(':');
                self.space();
                prop.traverse(self);
            }
        }
        false
    }

    fn enter_variable_declaration(&mut self, node: &mut VariableDeclaration) -> bool {
        node.pattern.traverse(self);

        if let Some(initializer) = node.initializer.as_mut() {
            self.string(" = ");
            initializer.traverse(self);
        }

        false
    }

    fn enter_literal(&mut self, node: &mut Literal) -> bool {
        match node {
            Literal::Null => self.string("null"),
            Literal::Boolean(true) => self.string("true"),
            Literal::Boolean(false) => self.string("false"),
            _ => return true,
        }

        false
    }

    fn enter_string_literal(&mut self, node: &mut LitString) -> bool {
        self.char(node.delimiter);
        self.string(&node.value);
        self.char(node.delimiter);
        false
    }

    fn enter_object_literal(&mut self, node: &mut Object) -> bool {
        if node.props.is_empty() {
            self.string("{}");
        } else {
            todo!()
        }
        false
    }

    fn enter_continue_stmt(&mut self, node: &mut StmtContinue) -> bool {
        self.string("continue");

        if node.label.is_some() {
            self.char(' ');
            node.label.traverse(self);
        }

        self.char(';');

        false
    }

    fn enter_break_stmt(&mut self, node: &mut StmtBreak) -> bool {
        self.string("break");

        if node.label.is_some() {
            self.char(' ');
            node.label.traverse(self);
        }

        self.char(';');
        false
    }

    fn exit_stmt_expr(&mut self, _node: &mut StmtExpr) {
        self.char(';');
    }

    fn exit_stmt(&mut self, _node: &mut Stmt) {
        if self.index.last_new_line() != self.pos() {
            self.new_line();
        }
    }

    fn enter_with_stmt(&mut self, node: &mut StmtWith) -> bool {
        self.string("with(");
        node.object.traverse(self);
        self.char(')');
        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_number(&mut self, node: &mut Number) -> bool {
        match node {
            Number::Integer(n, _) => self.string(&n.to_string()),
            Number::Decimal(_) => {}
        }

        false
    }

    fn enter_ident(&mut self, node: &mut Ident) -> bool {
        self.string(&node.name);
        false
    }

    fn enter_debugger_stmt(&mut self, _node: &mut StmtDebugger) -> bool {
        self.string("debugger");
        false
    }
}
