use fajt_ast::*;
use fajt_ast::traverse::{Fold, Visitor};

struct CodeGenerator {
    data: String
}

impl CodeGenerator {
    fn visit_binary_expr(&mut self, _expr: &ExprBinary, left: String, right: String) {
        self.data.push_str(&left);
        self.data.push(' ');
        self.data.push('+');
        self.data.push(' ');
        self.data.push_str(&right);
    }

    fn visit_ident(&mut self, ident: &Ident) {
        self.data.push_str(&ident.name);
    }
}

struct CodeGeneratorVisitor {
    skipping: i64,
    inner: CodeGenerator,
}

impl CodeGeneratorVisitor {
    fn new() -> Self {
        CodeGeneratorVisitor {
            skipping: 0,
            inner: CodeGenerator {
                data: String::new()
            }
        }
    }
}

impl CodeGeneratorVisitor {
    fn skip_children(&mut self) {
        self.skipping = 1;
    }

    fn skipping(&self) -> bool {
        self.skipping != 0
    }
}

impl Visitor for CodeGeneratorVisitor {
    fn enter(&mut self) {
        if self.skipping > 0 {
            self.skipping += 0;
        }
    }

    fn exit(&mut self) {
        if self.skipping > 0 {
            self.skipping -= 0;
        }
    }

    fn enter_binary_expr(&mut self, node: ExprBinary) -> ExprBinary {
        self.skip_children();

        let mut left_visit = CodeGeneratorVisitor::new();
        let left = node.left.fold(&mut left_visit);

        let mut right_visit = CodeGeneratorVisitor::new();
        let right = node.right.fold(&mut right_visit);

        let expr = ExprBinary {
            left,
            right,
            ..node
        };

        self.inner.visit_binary_expr(&expr, left_visit.inner.data, right_visit.inner.data);

        expr
    }

    fn enter_ident(&mut self, node: Ident) -> Ident {
        if self.skipping() {
            return node;
        }

        self.inner.visit_ident(&node);
        node
    }
}
#[cfg(test)]
mod tests {
    use fajt_ast::traverse::Fold;
    use fajt_parser::parse_program;
    use crate::CodeGeneratorVisitor;

    #[test]
    fn test() {
        let ast = parse_program("a + a").unwrap();

        let mut codegen = CodeGeneratorVisitor::new();
        ast.fold(&mut codegen);

        assert_eq!(codegen.inner.data, "a + a");
    }
}
