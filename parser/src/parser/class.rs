use crate::ast::{
    ClassElement, ClassMethod, ClassMethodKind, DeclClass, Expr, ExprClass, Ident, PropertyName,
    Stmt,
};
use crate::error::{Result, ThenTry};
use crate::{ContextModify, Parser};
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::{Span, Token};
use fajt_lexer::token_matches;

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

    fn parse_class_body(&mut self) -> Result<Vec<ClassElement>> {
        self.consume_assert(punct!("{"))?;
        let mut class_body = Vec::new();

        loop {
            let element: ClassElement = match self.current()? {
                token_matches!(punct!("}")) => {
                    self.consume()?;
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
                _ => {
                    let span_start = self.position();
                    let name = self.parse_property_name()?;
                    self.parse_class_method(span_start, name, ClassMethodKind::Method)?
                        .into()
                }
            };

            class_body.push(element);
        }

        Ok(class_body)
    }

    fn parse_class_get_set(&mut self, kind: ClassMethodKind) -> Result<ClassMethod> {
        let span_start = self.position();
        self.consume()?;
        let name = self.parse_property_name()?;
        self.parse_class_method(span_start, name, kind)
    }

    fn parse_class_generator_method(&mut self) -> Result<ClassMethod> {
        let span_start = self.position();
        self.consume()?;
        let name = self.parse_property_name()?;
        self.with_context(ContextModify::new().set_yield(true).set_await(false))
            .parse_class_method(span_start, name, ClassMethodKind::Method)
    }

    fn parse_class_async_method(&mut self) -> Result<ClassMethod> {
        let span_start = self.position();
        self.consume()?;

        let generator = self.maybe_consume(punct!("*"))?;
        let name = self.parse_property_name()?;
        self.with_context(ContextModify::new().set_yield(generator).set_await(true))
            .parse_class_method(span_start, name, ClassMethodKind::Method)
    }

    fn parse_class_method(
        &mut self,
        span_start: usize,
        name: PropertyName,
        kind: ClassMethodKind,
    ) -> Result<ClassMethod> {
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
            generator: self.context.is_yield,
            asynchronous: self.context.is_await,
        })
    }
}
