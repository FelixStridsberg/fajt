use crate::error::Result;
use fajt_ast::Expr;

pub trait IntoAssignmentPattern {
    // Converts any compatible Expr::* into Expr::AssignmentPattern, or returns
    // error if not compatible.
    fn try_into_assignment_pattern(self) -> Result<Expr>;
}

impl IntoAssignmentPattern for Expr {
    fn try_into_assignment_pattern(self) -> Result<Expr> {
        Ok(self)
    }
}
