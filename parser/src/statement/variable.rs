use crate::ast::{
    ArrayBinding, BindingIdentifier, BindingPattern, Expr, ObjectBinding, ObjectBindingProp, Stmt,
    VariableDeclaration, VariableKind, VariableStmt,
};
use crate::error::ErrorKind::UnexpectedToken;
use crate::error::{Error, Result};
use crate::Parser;
use fajt_lexer::punct;
use fajt_lexer::token::TokenValue;
use fajt_lexer::token_matches;
use std::convert::TryInto;

impl Parser<'_> {
    /// Parses the `VariableStatement` and `LexicalDeclaration` goal symbols.
    ///
    /// Example:
    /// var a = 2 + b, c = 2;
    /// ^~~~~~~~~~~~~~~~~~~~^
    pub(crate) fn parse_variable_statement(&mut self, variable_type: VariableKind) -> Result<Stmt> {
        let token = self.reader.consume()?;
        let start = token.span.start;

        // TODO parse all declarations
        let declarations = vec![self.parse_variable_declaration()?];
        let end = self.reader.position();

        Ok(VariableStmt::new(variable_type, declarations, (start, end)).into())
    }

    /// Parses the `VariableDeclaration` and `LexicalBinding` goal symbols.
    ///
    /// Example:
    /// var a = 2 + b, c = 2;
    ///     ^~~~~~~~^  ^~~~~^
    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration> {
        let token = self.reader.current()?;
        let span_start = token.span.start;

        let identifier = match &token.value {
            punct!("{") => self.parse_object_binding_pattern()?,
            punct!("[") => self.parse_array_binding_pattern()?,
            TokenValue::Identifier(_) => BindingPattern::Ident(BindingIdentifier::Ident(
                self.reader
                    .consume()?
                    .try_into()
                    .expect("Expected identifier"),
            )),
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
        let span = (span_start, span_end);
        Ok(VariableDeclaration::new(identifier, initializer, span))
    }

    /// Parses the `Initializer` goal symbol.
    ///
    /// Example:
    /// var a = 1 + 2, b = 100;
    ///         ^~~~^      ^~^
    fn parse_variable_initializer(&mut self) -> Result<Expr> {
        self.reader.consume()?; // Skip =
        self.parse_assignment_expression()
    }

    /// Parses the `ObjectBindingPattern` goal symbol.
    ///
    /// Example:
    /// var { a, b, ...rest} = c;
    ///     ^~~~~~~~~~~~~~~^
    fn parse_object_binding_pattern(&mut self) -> Result<BindingPattern> {
        let token = self.reader.consume()?;
        debug_assert_eq!(token.value, punct!("{"));

        let span_start = token.span.start;

        let mut bindings = Vec::new();

        let mut comma_allowed = false;
        loop {
            let token = self.reader.consume()?;

            // TODO key value and rest patterns
            match token {
                token_matches!(punct!("}")) => break,
                token_matches!(punct!(",")) if comma_allowed => comma_allowed = false,
                token_matches!(@ident) => {
                    comma_allowed = true;
                    bindings.push(ObjectBindingProp::Assign(BindingIdentifier::Ident(
                        token.try_into().unwrap(),
                    )))
                }
                t => return Err(Error::of(UnexpectedToken(t))),
            }
        }

        if self.reader.current()?.value == punct!(";") {
            self.reader.consume()?;
        }

        let span_end = self.reader.position();
        let span = (span_start, span_end);
        Ok(ObjectBinding::new(bindings, span).into())
    }

    /// Parses the `ArrayBindingPattern` goal symbol.
    ///
    /// Example:
    /// var [ a, b, ...rest ] = c;
    ///     ^~~~~~~~~~~~~~~~^
    fn parse_array_binding_pattern(&mut self) -> Result<BindingPattern> {
        let token = self.reader.consume()?;
        debug_assert_eq!(token.value, punct!("["));

        let span_start = token.span.start;

        let mut bindings = Vec::new();
        loop {
            let token = self.reader.consume()?;

            match token {
                token_matches!(punct!("]")) => break,
                token_matches!(@ident) => bindings.push(Some(BindingPattern::Ident(
                    BindingIdentifier::Ident(token.try_into().unwrap()),
                ))),
                t => return Err(Error::of(UnexpectedToken(t))),
            }
        }

        let span_end = self.reader.position();
        let span = (span_start, span_end);

        Ok(ArrayBinding::new(bindings, span).into())
    }
}
