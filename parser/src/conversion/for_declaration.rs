use crate::conversion::NormalizeAssignmentPattern;
use crate::error::{Error, Result};
use crate::static_semantics::ExprSemantics;
use crate::Context;
use fajt_ast::{Expr, ForBinding, ForDeclaration, ForInit};

pub trait TryIntoForDeclaration {
    fn try_into_for_declaration(self, context: &Context) -> Result<ForDeclaration>;
}

impl TryIntoForDeclaration for ForInit {
    fn try_into_for_declaration(self, context: &Context) -> Result<ForDeclaration> {
        match self {
            ForInit::Expr(expr) => {
                let assignment_expr = expr.normalize_assignment_pattern()?;
                if !matches!(assignment_expr, Expr::AssignmentPattern(_)) {
                    assignment_expr.early_errors_left_hand_side_expr(context)?;
                }
                Ok(ForDeclaration::Expr(Box::new(assignment_expr)))
            }
            ForInit::Declaration(mut decl) => {
                if decl.declarations.len() != 1 {
                    return Err(Error::syntax_error(
                        "Only one declaration is allowed in this context".to_owned(),
                        decl.span,
                    ));
                }

                let declaration = decl.declarations.pop().unwrap();
                if declaration.initializer.is_some() {
                    return Err(Error::syntax_error(
                        "Initializers are not allowed in this context".to_owned(),
                        decl.span,
                    ));
                }

                Ok(ForDeclaration::Declaration(ForBinding {
                    span: decl.span.clone(),
                    kind: decl.kind.clone(),
                    binding: declaration.pattern,
                }))
            }
        }
    }
}
