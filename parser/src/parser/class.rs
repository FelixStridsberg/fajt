use crate::ast::{ClassElement, ClassMethod, ClassMethodKind, Expr, ExprClass};
use crate::error::{Result, ThenTry};
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    /// Parses the `ClassExpression` goal symbol.
    pub(super) fn parse_class_expr(&mut self) -> Result<Expr> {
        let span_start = self.position();
        self.consume_assert(keyword!("class"))?;

        let identifier = self
            .is_identifier()
            .then(|| self.parse_identifier().unwrap());

        let super_class = self
            .maybe_consume(keyword!("extends"))?
            .then_try(|| self.parse_left_hand_side_expr().map(Box::new))?;

        let body = self.parse_class_body()?;
        let span = self.span_from(span_start);
        Ok(ExprClass {
            span,
            identifier,
            super_class,
            body,
        }
        .into())
    }

    fn parse_class_body(&mut self) -> Result<Vec<ClassElement>> {
        self.consume_assert(punct!("{"))?;
        let mut class_body = Vec::new();

        loop {
            let element: ClassElement = match self.reader.current()? {
                token_matches!(punct!("}")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!("*")) => self.parse_class_generator_method()?.into(),
                token_matches!(keyword!("async")) if !self.followed_by_new_lined() => {
                    self.parse_class_async_method()?.into()
                }
                token_matches!(keyword!("get")) => {
                    self.parse_class_get_set(ClassMethodKind::Get)?.into()
                }
                token_matches!(keyword!("set")) => {
                    self.parse_class_get_set(ClassMethodKind::Set)?.into()
                }
                _ => self
                    .parse_class_method(self.position(), ClassMethodKind::Method, false, false)?
                    .into(),
            };

            class_body.push(element);
        }

        Ok(class_body)
    }

    fn parse_class_get_set(&mut self, kind: ClassMethodKind) -> Result<ClassMethod> {
        let span_start = self.position();
        self.reader.consume()?;
        self.parse_class_method(span_start, kind, false, false)
    }

    fn parse_class_generator_method(&mut self) -> Result<ClassMethod> {
        let span_start = self.position();
        self.reader.consume()?;
        self.parse_class_method(span_start, ClassMethodKind::Method, true, false)
    }

    fn parse_class_async_method(&mut self) -> Result<ClassMethod> {
        let span_start = self.position();
        self.reader.consume()?;

        let generator = self.maybe_consume(punct!("*"))?;
        self.parse_class_method(span_start, ClassMethodKind::Method, generator, true)
    }

    fn parse_class_method(
        &mut self,
        span_start: usize,
        kind: ClassMethodKind,
        generator: bool,
        asynchronous: bool,
    ) -> Result<ClassMethod> {
        let name = self.parse_property_name()?;
        // TODO this should be `UniqueFormalParameters` or `PropertySetParameterList` depending on kind.
        let parameters = self.parse_formal_parameters()?;
        let body = self.parse_function_body()?;

        let span = self.span_from(span_start);
        Ok(ClassMethod {
            span,
            name,
            kind,
            parameters,
            body,
            generator,
            asynchronous,
        })
    }
}
