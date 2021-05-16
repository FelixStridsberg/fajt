use crate::ast::{
    ArrayBinding, BindingPattern, Expression, Ident, ObjectBinding, ObjectBindingProp, Statement,
    VariableDeclaration, VariableKind, VariableStatement,
};
use crate::error::ErrorKind::{SyntaxError, UnexpectedToken};
use crate::error::{Error, Result};
use crate::Parser;
use fajt_lexer::punct;
use fajt_lexer::token::Punct::{BraceClose, BracketClose};
use fajt_lexer::token::{Punct, Span, TokenValue};
use fajt_lexer::token_matches;

impl Parser<'_, '_> {
    /// Parses the `VariableStatement` and `LexicalDeclaration` goal symbols.
    ///
    /// Example:
    /// ```no_rust
    /// var a = 2 + b, c = 2;
    /// ^~~~~~~~~~~~~~~~~~~~^
    /// ```
    pub(crate) fn parse_variable_statement(&mut self, kind: VariableKind) -> Result<Statement> {
        let token = self.reader.consume()?;
        let start = token.span.start;

        // TODO parse all declarations
        let declarations = vec![self.parse_variable_declaration()?];
        let end = self.reader.position();

        let span = Span::new(start, end);
        Ok(VariableStatement {
            span,
            kind,
            declarations,
        }
        .into())
    }

    /// Parses the `VariableDeclaration` and `LexicalBinding` goal symbols.
    ///
    /// Example:
    /// ```no_rust
    /// var a = 2 + b, c = 2;
    ///     ^~~~~~~~^  ^~~~~^
    /// ```
    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let token = self.reader.current()?;
        let span_start = token.span.start;

        let identifier = match &token.value {
            punct!("{") => self.parse_object_binding_pattern()?,
            punct!("[") => self.parse_array_binding_pattern()?,
            _ if self.is_identifier()? => BindingPattern::Ident(self.parse_identifier()?),
            _ => return Err(Error::of(UnexpectedToken(self.reader.consume()?))),
        };

        let initializer = match self.reader.current()? {
            token_matches!(punct!("=")) => Some(self.parse_variable_initializer()?),
            token_matches!(punct!(";")) => {
                self.reader.consume()?;
                None
            }
            _ => None,
        };

        let span_end = self.reader.position();
        let span = Span::new(span_start, span_end);
        Ok(VariableDeclaration {
            span,
            identifier,
            initializer,
        })
    }

    /// Parses the `Initializer` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// var a = 1 + 2, b = 100;
    ///         ^~~~^      ^~^
    /// ```
    fn parse_variable_initializer(&mut self) -> Result<Expression> {
        self.reader.consume()?; // Skip =
        self.parse_assignment_expression()
    }

    /// Parses the `ObjectBindingPattern` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// var { a, b, ...rest} = c;
    ///     ^~~~~~~~~~~~~~~^
    /// ```
    fn parse_object_binding_pattern(&mut self) -> Result<BindingPattern> {
        let token = self.reader.consume()?;
        debug_assert_eq!(token.value, punct!("{"));

        let span_start = token.span.start;

        let mut props = Vec::new();

        let mut rest = None;
        loop {
            match self.reader.current()? {
                token_matches!(punct!("}")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!("...")) => {
                    self.reader.consume()?;
                    rest = self.parse_rest_binding_ident(BracketClose)?;
                    break;
                }
                _ if self.is_identifier()? => {
                    let ident = self.parse_identifier()?;
                    props.push(ObjectBindingProp::Assign(ident));
                    self.consume_object_delimiter()?;
                }
                _ => return Err(Error::of(UnexpectedToken(self.reader.consume()?))),
            }
        }

        if self.reader.current()?.value == punct!(";") {
            self.reader.consume()?;
        }

        let span_end = self.reader.position();
        let span = Span::new(span_start, span_end);

        Ok(ObjectBinding { span, props, rest }.into())
    }

    /// Parses the `ArrayBindingPattern` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// var [ a, b, ...rest ] = c;
    ///     ^~~~~~~~~~~~~~~~^
    /// ```
    fn parse_array_binding_pattern(&mut self) -> Result<BindingPattern> {
        let token = self.reader.consume()?;
        debug_assert_eq!(token.value, punct!("["));

        let span_start = token.span.start;

        let mut elements = Vec::new();

        let mut rest = None;
        loop {
            match self.reader.current()? {
                token_matches!(punct!("]")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!("{")) => {
                    elements.push(Some(self.parse_object_binding_pattern()?));
                    self.consume_array_delimiter()?;
                }
                token_matches!(punct!(",")) => {
                    self.reader.consume()?;
                    elements.push(None);
                }
                token_matches!(punct!("...")) => {
                    self.reader.consume()?;
                    rest = self.parse_rest_binding_ident(BraceClose)?;
                    break;
                }
                _ if self.is_identifier()? => {
                    let ident = self.parse_identifier()?;
                    elements.push(Some(BindingPattern::Ident(ident)));
                    self.consume_array_delimiter()?;
                }
                _ => return Err(Error::of(UnexpectedToken(self.reader.consume()?))),
            }
        }

        let span_end = self.reader.position();
        let span = Span::new(span_start, span_end);

        Ok(ArrayBinding {
            span,
            elements,
            rest,
        }
        .into())
    }

    /// Parses the BindingIdentifier goal symbol.
    /// This also consumes the expected end punctuator.
    ///
    /// Examples:
    /// ```no_rust
    /// var { ...rest } = a;
    ///          ^~~~~^
    /// var [ ...rest ] = a;
    ///          ^~~~~^
    /// ```
    fn parse_rest_binding_ident(&mut self, expected_end: Punct) -> Result<Option<Ident>> {
        let ident = self.parse_identifier()?;
        let end_token = self.reader.consume()?;

        if let TokenValue::Punct(p) = end_token.value {
            if p == expected_end {
                return Ok(Some(ident));
            }
        }

        Err(Error::of(SyntaxError(
            "Rest element must be last element".to_owned(),
            ident.span,
        )))
    }
}
