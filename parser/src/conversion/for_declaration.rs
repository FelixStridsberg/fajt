use crate::conversion::IntoAssignmentPattern;
use crate::error::Result;
use crate::static_semantics::ExprSemantics;
use crate::Context;
use fajt_ast::{AssignmentOperator, ForBinding, ForDeclaration, ForInit};

pub trait TryIntoForDeclaration {
    fn try_into_for_declaration(self, context: &Context) -> Result<ForDeclaration>;
}

impl TryIntoForDeclaration for ForInit {
    fn try_into_for_declaration(self, context: &Context) -> Result<ForDeclaration> {
        match self {
            ForInit::Expr(expr) => {
                expr.early_errors_left_hand_side_expr(context, &AssignmentOperator::Assign)?;
                Ok(ForDeclaration::Expr(Box::new(
                    expr.try_into_assignment_pattern()?,
                )))
            }
            // TODO validate only one declaration
            ForInit::Declaration(decl) => Ok(ForDeclaration::Declaration(ForBinding {
                span: decl.span.clone(),
                kind: decl.kind.clone(),
                // TODO
                binding: decl.declarations.into_iter().next().unwrap().pattern,
            })),
        }
    }
}
