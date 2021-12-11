use fajt_ast::{BindingElement, Body, DeclFunction, Expr, ExprBinary, ExprLiteral, FormalParameters, Ident, Literal, Number, Program, StatementList, Stmt, StmtExpr, StmtReturn};
use fajt_ast::traverse::{Fold, Visitor};

struct CodegenVisitor {
    pub result: String,
}

impl CodegenVisitor {
    fn new() -> Self {
        Self {
            result: String::new()
        }
    }
}

impl Visitor for CodegenVisitor {
    fn enter_binary_expr(&mut self, node: ExprBinary) -> ExprBinary {
        let left = node.left.fold(self);

        ExprBinary {
            left,
            ..node
        }
    }

    fn enter_literal_expr(&mut self, node: ExprLiteral) -> ExprLiteral {
        match &node.literal {
            Literal::Number(number) => {
                match number {
                    Number::Integer(num, _) => {
                        self.result.push_str(&num.to_string());
                    }
                    x => {
                        println!("Not implemented {:?}", x);
                    }
                }
            }
            x => {
                println!("Not implemented {:?}", x);
            }
        }
        node
    }
}


#[cfg(test)]
mod tests {
    use fajt_ast::traverse::{Fold, TraceVisitor};
    use fajt_parser::parse_program;
    use crate::CodegenVisitor;

    #[test]
    fn it_works() {
        let ast = parse_program("1 + 1").unwrap();


        let mut trace = TraceVisitor::new();
        let mut codegen = CodegenVisitor::new();

        ast.fold(&mut codegen);
        println!("{:?}", trace.visits);

        assert_eq!(codegen.result, "1 + 1");
    }
}
