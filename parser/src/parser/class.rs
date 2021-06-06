use crate::ast::{ClassElement, ClassExpression, ClassMethod, ClassMethodKind, Expression};
use crate::error::{ErrorKind, Result};
use crate::Parser;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token_matches;

impl Parser<'_, '_> {
    /// Parses the `ClassExpression` goal symbol.
    pub(super) fn parse_class_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("class")));

        let identifier = self
            .is_identifier()
            .then(|| self.parse_identifier().unwrap());

        let super_class = if token_matches!(self.reader.current(), ok: keyword!("extends")) {
            self.reader.consume()?;
            Some(self.parse_left_hand_side_expression()?)
        } else {
            None
        };

        let body = self.parse_class_body()?;
        let span = self.span_from(span_start);
        Ok(ClassExpression {
            span,
            identifier,
            super_class,
            body,
        }
        .into())
    }

    fn parse_class_body(&mut self) -> Result<Vec<ClassElement>> {
        if !token_matches!(self.reader.current(), ok: punct!("{")) {
            return err!(ErrorKind::UnexpectedToken(self.reader.consume()?));
        }

        self.reader.consume()?;
        let mut class_body = Vec::new();

        loop {
            match self.reader.current()? {
                token_matches!(punct!("}")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!("*")) => {
                    let span_start = self.position();
                    self.reader.consume()?;
                    class_body.push(
                        self.parse_class_method(span_start, ClassMethodKind::Method, true, false)?
                            .into(),
                    )
                }
                token_matches!(keyword!("async")) => {
                    // TODO [no LineTerminator here] after async
                    let span_start = self.position();
                    self.reader.consume()?;

                    let generator = token_matches!(self.reader.current(), ok: punct!("*"));
                    if generator {
                        self.reader.consume()?;
                    }

                    class_body.push(
                        self.parse_class_method(
                            span_start,
                            ClassMethodKind::Method,
                            generator,
                            true,
                        )?
                        .into(),
                    )
                }
                token_matches!(keyword!("get")) => {
                    let span_start = self.position();
                    self.reader.consume()?;
                    class_body.push(
                        self.parse_class_method(span_start, ClassMethodKind::Get, false, false)?
                            .into(),
                    )
                }
                token_matches!(keyword!("set")) => {
                    let span_start = self.position();
                    self.reader.consume()?;
                    class_body.push(
                        self.parse_class_method(span_start, ClassMethodKind::Set, false, false)?
                            .into(),
                    )
                }
                _ => class_body.push(
                    self.parse_class_method(
                        self.position(),
                        ClassMethodKind::Method,
                        false,
                        false,
                    )?
                    .into(),
                ),
            }
        }

        Ok(class_body)
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
