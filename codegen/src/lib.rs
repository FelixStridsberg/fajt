use fajt_ast::traverse::{Traverse, Visitor};
use fajt_ast::*;

pub fn generate_code(mut program: Program) -> String {
    let mut codegen = CodeGenerator::new();
    program.traverse(&mut codegen);
    codegen.to_string()
}

struct CodeGenerator {
    indent: u32,
    data: String,
    last_new_line: usize,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            indent: 0,
            data: String::new(),
            last_new_line: 0,
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

    fn new_line(&mut self) -> &mut Self {
        self.push('\n');
        self.last_new_line = self.data.len();
        self
    }

    fn maybe_indent(&mut self) {
        if self.should_indent() {
            self.data.push_str(&" ".repeat((4 * self.indent) as usize));
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
        self.push_str("function ");
        node.identifier.traverse(self);
        node.parameters.traverse(self);
        self.space();
        node.body.traverse(self);
        false
    }

    fn enter_body(&mut self, node: &mut Body) -> bool {
        self.push('{').new_line();
        self.indent();
        node.statements.traverse(self);
        self.dedent();
        self.push('}');
        self.new_line();
        false
    }

    fn enter_return_stmt(&mut self, node: &mut StmtReturn) -> bool {
        self.push_str("return ");
        node.argument.traverse(self);
        self.push(';');
        self.new_line();
        false
    }

    fn enter_format_parameters(&mut self, node: &mut FormalParameters) -> bool {
        self.push('(');
        for bind in node.bindings.iter_mut() {
            bind.traverse(self);
        }
        self.push(')');
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
