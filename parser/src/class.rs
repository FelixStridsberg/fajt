use crate::error::{Result, ThenTry};
use crate::Parser;
use fajt_ast::{ClassElement, DeclClass, Expr, ExprClass, Ident, Span, Stmt};
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    /// Parses the `ClassDeclaration` goal symbol.
    pub(super) fn parse_class_decl(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("class"))?;

        let identifier = if self.context.is_default && self.current_matches(punct!("{")) {
            let current = self.current().unwrap();
            Ident::new("", Span::new(current.span.start, current.span.start))
        } else {
            self.parse_identifier()?
        };

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

    /// Parses the `ClassExpression` goal symbol.
    pub(super) fn parse_class_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(keyword!("class"))?;

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

    /// Parses the `ClassTail` goal symbol, returns (ClassHeritage, ClassBody).
    fn parse_class_tail(&mut self) -> Result<(Option<Box<Expr>>, Vec<ClassElement>)> {
        let super_class = self
            .maybe_consume(keyword!("extends"))?
            .then_try(|| self.parse_left_hand_side_expr().map(Box::new))?;

        let body = self.parse_class_body()?;
        Ok((super_class, body))
    }

    /// Parses the `ClassBody` goal symbol, including the { and } terminals.
    fn parse_class_body(&mut self) -> Result<Vec<ClassElement>> {
        self.consume_assert(punct!("{"))?;
        let mut class_body = Vec::new();

        loop {
            if self.current_matches(punct!("}")) {
                self.consume()?;
                break;
            }

            let element: ClassElement = self.parse_method_definition()?.into();
            class_body.push(element);
        }

        Ok(class_body)
    }
}
