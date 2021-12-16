use fajt_ast::traverse::{Traverse, Visitor};
use fajt_ast::*;

const INDENTATION_SIZE: usize = 4;

pub fn generate_code(mut program: Program) -> String {
    let mut codegen = CodeGenerator::new();
    program.traverse(&mut codegen);
    codegen.to_string()
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

    fn to_string(self) -> String {
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
        self.data.len() != 0 && self.data.len() == self.last_new_line
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
        node.identifier.traverse(self);
        node.parameters.traverse(self);
        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_body(&mut self, node: &mut Body) -> bool {
        if node.statements.is_empty() {
            self.push_str("{}").new_line();
            return false;
        }

        self.block_start();
        self.indent();
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
            if let Some(_) = bindings.peek() {
                self.push(',');
                self.space();
            }
        }
        self.push(')');
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

            if let Some(_) = declarations.peek() {
                self.push(',');
            } else {
                self.push(';');
            }

            self.new_line();
        }

        self.stop_align();

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

#[cfg(test)]
mod tests {
    use crate::{generate_code, CodeGenerator};
    use fajt_ast::traverse::Traverse;
    use fajt_parser::parse_program;

    #[test]
    fn add_expr() {
        let mut ast = parse_program("a + a").unwrap();
        let code = generate_code(ast);

        assert_eq!(code, "a + a");
    }

    #[test]
    fn parenthesized_expr() {
        let mut ast = parse_program("(a + a)").unwrap();

        let code = generate_code(ast);
        assert_eq!(code, "(a + a)");
    }

    #[test]
    fn function() {
        let input = "function plus(n) {\n    return n + n;\n}";
        let mut ast = parse_program(input).unwrap();

        let code = generate_code(ast);
        assert_eq!(code, input);
    }
}
