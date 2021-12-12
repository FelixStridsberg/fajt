use fajt_ast::*;
use fajt_ast::traverse::{Traverse, Visitor};

struct CodeGenerator {
    data: String
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            data: String::new()
        }
    }

}

impl Visitor for CodeGenerator {
    fn enter_binary_expr(&mut self, node: &mut ExprBinary) -> bool {
        node.left.traverse(self);
        self.data.push_str(" + ");
        node.right.traverse(self);
        false
    }

    fn enter_ident(&mut self, node: &mut Ident) -> bool {
        self.data.push_str(&node.name);
        true
    }
}
#[cfg(test)]
mod tests {
    use fajt_ast::traverse::Traverse;
    use fajt_parser::parse_program;
    use crate::CodeGenerator;

    #[test]
    fn test() {
        let mut ast = parse_program("a + a").unwrap();
        let mut codegen = CodeGenerator::new();
        ast.traverse(&mut codegen);

        assert_eq!(codegen.data, "a + a");
    }
}
