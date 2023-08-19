use crate::error::{Error, Result};
use crate::static_semantics::ExprSemantics;
use crate::Context;
use fajt_ast::{
    ArrayAssignmentPattern, ArrayElement, AssignmentElement, AssignmentPattern, AssignmentProp,
    Expr, ExprLiteral, LitArray, LitObject, Literal, NamedAssignmentProp, ObjectAssignmentPattern,
    PropertyDefinition, SingleNameAssignmentProp, Span, Spanned,
};

pub trait NormalizeAssignmentPattern {
    // Converts any compatible Expr::* into Expr::AssignmentPattern.
    fn normalize_assignment_pattern(self, context: &Context) -> Result<Expr>;
}

impl NormalizeAssignmentPattern for Expr {
    fn normalize_assignment_pattern(self, context: &Context) -> Result<Expr> {
        match self {
            Expr::Literal(literal) => convert_literal(context, literal),
            _ => Ok(self),
        }
    }
}

fn convert_literal(context: &Context, expr: ExprLiteral) -> Result<Expr> {
    match expr.literal {
        Literal::Array(array) => convert_array_literal(context, expr.span, array),
        Literal::Object(object) => convert_object_literal(context, expr.span, object),
        _ => Ok(Expr::Literal(expr)),
    }
}

fn convert_array_literal(context: &Context, span: Span, array: LitArray) -> Result<Expr> {
    let mut pattern = ArrayAssignmentPattern {
        span,
        elements: Vec::with_capacity(array.elements.len()),
        rest: None,
    };

    let mut elements = array.elements.into_iter().peekable();
    while let Some(element) = elements.next() {
        match element {
            ArrayElement::Elision => pattern.elements.push(None),
            ArrayElement::Spread(expr) => {
                if elements.peek().is_some() {
                    return Err(Error::syntax_error(
                        "Rest element must be last element".to_owned(),
                        expr.span().clone(),
                    ));
                }

                pattern.rest = Some(Box::new(expr));
            }
            ArrayElement::Expr(expr) => {
                let span = expr.span().clone();
                let (target, initializer) = expr_to_assignment_element(context, expr)?;

                if !matches!(*target, Expr::AssignmentPattern(_))
                    && !target.is_assignment_target_type_simple(context)?
                {
                    return Err(Error::syntax_error(
                        "Invalid destructuring target".to_owned(),
                        span,
                    ));
                }

                pattern.elements.push(Some(AssignmentElement {
                    span,
                    target,
                    initializer,
                }))
            }
        }
    }

    Ok(Expr::AssignmentPattern(AssignmentPattern::Array(pattern)))
}

fn convert_object_literal(context: &Context, span: Span, object: LitObject) -> Result<Expr> {
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
            Named(property) => {
                let (value, initializer) = expr_to_assignment_element(context, property.value)?;

                pattern
                    .props
                    .push(AssignmentProp::Named(NamedAssignmentProp {
                        span: property.span.clone(),
                        name: property.name,
                        value,
                        initializer,
                    }))
            }
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

fn expr_to_assignment_element(
    context: &Context,
    expr: Expr,
) -> Result<(Box<Expr>, Option<Box<Expr>>)> {
    let expr = match expr {
        Expr::Assignment(assign) => return Ok((assign.left, Some(assign.right))),
        Expr::Literal(literal) => convert_literal(context, literal)?,
        _ => expr,
    };

    Ok((Box::new(expr), None))
}
