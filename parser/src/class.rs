use crate::error::Result;
use crate::{Parser, ThenTry};
use fajt_ast::{ClassElement, DeclClass, Expr, ExprClass, Ident, Stmt};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::{keyword, LexerState};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses the `ClassDeclaration` production.
    pub(super) fn parse_class_decl(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("class"))?;

        let identifier = self.parse_optional_class_identifier()?;
        let (super_class, body) = self.parse_class_tail()?;

        let span = self.span_from(span_start);
        Ok(DeclClass {
            span,
            identifier,
            super_class,
            body,
        }
        .into())
    }

    /// Parses optional class identifier, returns dummy if identifier is optional and missing.
    fn parse_optional_class_identifier(&mut self) -> Result<Ident> {
        if self.context.is_default && self.current_matches(&punct!("{")) {
            let current = self.current().unwrap();
            Ok(Ident::dummy(current.span.start))
        } else {
            self.parse_identifier()
        }
    }

    /// Parses the `ClassExpression` production.
    pub(super) fn parse_class_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(&keyword!("class"))?;

        let identifier = self
            .is_identifier()
            .then(|| self.parse_identifier().unwrap());

        let (super_class, body) = self.parse_class_tail()?;
        let span = self.span_from(span_start);
        Ok(ExprClass {
            span,
            identifier,
            super_class,
            body,
        }
        .into())
    }

    /// Parses the `ClassTail` production, returns (ClassHeritage, ClassBody).
    fn parse_class_tail(&mut self) -> Result<(Option<Box<Expr>>, Vec<ClassElement>)> {
        let super_class = self
            .maybe_consume(&keyword!("extends"))?
            .then_try(|| self.parse_left_hand_side_expr().map(Box::new))?;

        let has_super = super_class.is_some();
        let body = self.parse_class_body(has_super)?;
        Ok((super_class, body))
    }

    /// Parses the `ClassBody` production, including the { and } terminals.
    fn parse_class_body(&mut self, has_super: bool) -> Result<Vec<ClassElement>> {
        self.consume_assert(&punct!("{"))?;
        let mut class_body = Vec::new();

        loop {
            if self.current_matches(&punct!("}")) {
                self.consume()?;
                break;
            }

            class_body.push(self.parse_class_element(has_super)?);
        }

        Ok(class_body)
    }

    /// Parses the `ClassElement` production.
    fn parse_class_element(&mut self, has_super: bool) -> Result<ClassElement> {
        let super_call_allowed = has_super && self.current_matches_identifier("constructor");
        let context = self
            .context
            .with_in_method(true)
            .with_super_call_allowed(super_call_allowed);

        Ok(self.with_context(context).parse_method_definition()?.into())
    }
}
