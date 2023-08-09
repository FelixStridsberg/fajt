use crate::error::{Error, Result};
use fajt_ast::{
    AssignmentPattern, AssignmentProp, Expr, ExprLiteral, LitObject, Literal, NamedAssignmentProp,
    ObjectAssignmentPattern, PropertyDefinition, SingleNameAssignmentProp, Span, Spanned,
};

pub trait IntoAssignmentPattern {
    // Converts any compatible Expr::* into Expr::AssignmentPattern, or returns
    // error if not compatible.
    fn try_into_assignment_pattern(self) -> Result<Expr>;
}

impl IntoAssignmentPattern for Expr {
    fn try_into_assignment_pattern(self) -> Result<Expr> {
        match self {
            Expr::Literal(literal) => convert_literal(literal),
            _ => Ok(self),
        }
    }
}

fn convert_literal(expr: ExprLiteral) -> Result<Expr> {
    match expr.literal {
        Literal::Object(object) => convert_object_literal(expr.span, object),
        _ => Ok(Expr::Literal(expr)),
    }
}

fn convert_object_literal(span: Span, object: LitObject) -> Result<Expr> {
    use PropertyDefinition::*;

    let mut pattern = ObjectAssignmentPattern {
        span,
        props: Vec::with_capacity(object.props.len()),
        rest: None,
    };

    let mut props = object.props.into_iter().peekable();
    while let Some(prop) = props.next() {
        match prop {
            IdentRef(ident) => {
                pattern
                    .props
                    .push(AssignmentProp::Single(SingleNameAssignmentProp {
                        span: ident.span.clone(),
                        ident,
                        initializer: None,
                    }))
            }
            Named(property) => pattern
                .props
                .push(AssignmentProp::Named(NamedAssignmentProp {
                    span: property.span.clone(),
                    name: property.name,
                    value: Box::new(property.value),
                })),
            Spread(expr) => {
                if props.peek().is_some() {
                    return Err(Error::syntax_error(
                        "Rest element must be last element".to_owned(),
                        expr.span().clone(),
                    ));
                }

                pattern.rest = Some(Box::new(expr));
            }
            Method(method) => {
                return Err(Error::syntax_error(
                    "Invalid destructuring assignment target".to_owned(),
                    method.span.clone(),
                ));
            }
        }
    }

    Ok(Expr::AssignmentPattern(AssignmentPattern::Object(pattern)))
}

