use crate::error::Error;
use crate::error::Result;
use crate::static_semantics::ExprSemantics;
use crate::Parser;
use crate::ThenTry;
use fajt_ast::{
    ArrayAssignmentPattern, AssignmentElement, AssignmentPattern, AssignmentProp, Expr,
    NamedAssignmentProp, ObjectAssignmentPattern, SingleNameAssignmentProp, Spanned,
};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use fajt_lexer::LexerState;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    pub(super) fn parse_assignment_pattern(&mut self) -> Result<Expr> {
        match self.current()? {
            token_matches!(punct!("[")) => self
                .with_context(self.context.inside_assignment_expr(true))
                .parse_array_assignment_pattern(),
            token_matches!(punct!("{")) => self
                .with_context(self.context.inside_assignment_expr(true))
                .parse_object_assignment_pattern(),
            _ => Ok(Expr::IdentRef(self.parse_identifier()?)),
        }
    }

    pub(super) fn parse_array_assignment_pattern(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&punct!("["))?;

        let mut elements = Vec::new();
        let mut rest = None;

        loop {
            match self.current()? {
                token_matches!(punct!("]")) => {
                    self.consume()?;
                    break;
                }
                token_matches!(punct!("...")) => {
                    self.consume()?;
                    let rest_expr = self.parse_left_hand_side_expr()?;

                    if !self.maybe_consume(&punct!("]"))? {
                        return Err(Error::syntax_error(
                            "Rest element must be last element".to_owned(),
                            rest_expr.span().clone(),
                        ));
                    }

                    rest = Some(Box::new(rest_expr));
                    break;
                }
                token_matches!(punct!(",")) => {
                    elements.push(None);
                    self.consume()?;
                }
                _ => {
                    elements.push(Some(self.parse_assignment_element()?));
                }
            }
        }

        let span = self.span_from(span_start);
        Ok(Expr::AssignmentPattern(AssignmentPattern::Array(
            ArrayAssignmentPattern {
                span,
                elements,
                rest,
            },
        )))
    }

    fn parse_assignment_element(&mut self) -> Result<AssignmentElement> {
        let span_start = self.position();
        let target = Box::new(self.parse_left_hand_side_expr()?);

        if !matches!(*target, Expr::AssignmentPattern(_))
            && !target.is_assignment_target_type_simple(&self.context)?
        {
            return Err(Error::syntax_error(
                "Invalid destructuring assignment target".to_owned(),
                target.span().clone(),
            ));
        }

        let initializer = if self.maybe_consume(&punct!("="))? {
            Some(Box::new(
                self.with_context(self.context.with_in(true))
                    .parse_assignment_expr()?,
            ))
        } else {
            None
        };

        let span = self.span_from(span_start);
        Ok(AssignmentElement {
            span,
            target,
            initializer,
        })
    }

    /// Parses the `ObjectAssignmentPattern` production.
    pub(super) fn parse_object_assignment_pattern(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&punct!("{"))?;

        let mut props = Vec::new();

        let mut rest = None;
        loop {
            match self.current()? {
                token_matches!(punct!("}")) => {
                    self.consume()?;
                    break;
                }
                token_matches!(punct!("...")) => {
                    self.consume()?;
                    let rest_expr = self.parse_left_hand_side_expr()?;
                    if !self.maybe_consume(&punct!("}"))? {
                        return Err(Error::syntax_error(
                            "Rest element must be last element".to_owned(),
                            rest_expr.span().clone(),
                        ));
                    }

                    rest = Some(Box::new(rest_expr));
                    break;
                }
                token if token_matches!(token, punct!("[")) || self.peek_matches(&punct!(":")) => {
                    props.push(AssignmentProp::Named(self.parse_named_assignment_prop()?));
                    self.consume_list_delimiter(&punct!("}"))?;
                }
                _ => {
                    props.push(AssignmentProp::Single(
                        self.parse_single_name_assignment_prop()?,
                    ));
                    self.consume_list_delimiter(&punct!("}"))?;
                }
            }
        }

        let span = self.span_from(span_start);
        Ok(Expr::AssignmentPattern(AssignmentPattern::Object(
            ObjectAssignmentPattern { span, props, rest },
        )))
    }

    /// Parses the `PropertyName: AssignmentElement` case of the `AssignmentProperty` production.
    fn parse_named_assignment_prop(&mut self) -> Result<NamedAssignmentProp> {
        let span_start = self.position();
        let name = self.parse_property_name()?;
        self.consume_assert(&punct!(":"))?;

        let value = Box::new(self.parse_left_hand_side_expr()?);
        let initializer = self
            .current_matches(&punct!("="))
            .then_try(|| self.parse_initializer())?;

        let span = self.span_from(span_start);
        Ok(NamedAssignmentProp {
            span,
            name,
            value,
            initializer: initializer.map(Box::new),
        })
    }

    /// Parses the `SingleNameBinding` production.
    fn parse_single_name_assignment_prop(&mut self) -> Result<SingleNameAssignmentProp> {
        let span_start = self.position();

        let ident = self.parse_identifier()?;
        let initializer = self
            .current_matches(&punct!("="))
            .then_try(|| self.parse_initializer())?;

        let span = self.span_from(span_start);
        Ok(SingleNameAssignmentProp {
            span,
            ident,
            initializer: initializer.map(Box::new),
        })
    }
}
