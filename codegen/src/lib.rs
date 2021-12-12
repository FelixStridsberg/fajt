use fajt_ast::*;
use fajt_ast::traverse::{Traverse, Visitor};

pub fn generate_code(mut program: Program) -> String {
    let mut codegen = CodeGenerator::new();
    program.traverse(&mut codegen);
    codegen.to_string()
}

struct CodeGenerator {
    indent: u32,
    data: String,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            indent: 0,
            data: String::new(),
        }
    }

    fn to_string(self) -> String {
        self.data
    }
}

impl Visitor for CodeGenerator {
    fn enter_binary_expr(&mut self, node: &mut ExprBinary) -> bool {
        node.left.traverse(self);
        self.data.push(' ');
        self.data.push_str(&node.operator.to_string());
        self.data.push(' ');
        node.right.traverse(self);
        false
    }

    fn enter_parenthesized_expr(&mut self, node: &mut ExprParenthesized) -> bool {
        self.data.push('(');
        node.expression.traverse(self);
        self.data.push(')');
        false
    }

    fn enter_function_decl(&mut self, node: &mut DeclFunction) -> bool {
        self.data.push_str("function ");
        node.identifier.traverse(self);
        node.parameters.traverse(self);
        self.data.push(' ');
        node.body.traverse(self);
        false
    }

    fn enter_body(&mut self, node: &mut Body) -> bool {
        self.data.push_str("{\n");
        self.indent += 1;
        node.statements.traverse(self);
        self.indent -= 1;
        self.data.push_str("\n}");
        false
    }

    fn enter_return_stmt(&mut self, node: &mut StmtReturn) -> bool {
        self.data.push_str(&" ".repeat((4 * self.indent) as usize));

        self.data.push_str("return ");
        node.argument.traverse(self);
        self.data.push(';');
        false
    }

    fn enter_format_parameters(&mut self, node: &mut FormalParameters) -> bool {
        self.data.push('(');
        for bind in node.bindings.iter_mut() {
            bind.traverse(self);
        }
        self.data.push(')');
        false
    }

    fn enter_ident(&mut self, node: &mut Ident) -> bool {
        self.data.push_str(&node.name);
        false
    }
}

#[cfg(test)]
mod tests {
    use fajt_ast::traverse::Traverse;
    use fajt_parser::parse_program;
    use crate::{CodeGenerator, generate_code};

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
