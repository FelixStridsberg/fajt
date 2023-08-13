use crate::error::Result;
use crate::Parser;
use crate::ThenTry;
use fajt_ast::{
    AssignmentPattern, AssignmentProp, Expr, NamedAssignmentProp, ObjectAssignmentPattern,
    SingleNameAssignmentProp,
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
                    rest = Some(Box::new(self.parse_left_hand_side_expr()?));
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
