use crate::ast::{
    ArrayBinding, BindingPattern, Expr, Ident, ObjectBinding, ObjectBindingProp, Stmt,
    VariableDeclaration, VariableKind, VariableStmt,
};
use crate::error::ErrorKind::{SyntaxError, UnexpectedToken};
use crate::error::{Error, Result};
use crate::Parser;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Punct::{BraceClose, BracketClose};
use fajt_lexer::token::{Punct, TokenValue};
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
            TokenValue::Identifier(_) => BindingPattern::Ident(
                self.reader
                    .consume()?
                    .try_into()
                    .expect("Expected identifier"),
            ),
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

        let mut rest = None;
        let mut comma_allowed = false;
        loop {
            match self.reader.current()? {
                token_matches!(punct!("}")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!(",")) if comma_allowed => {
                    self.reader.consume()?;
                    comma_allowed = false
                }
                token_matches!(punct!("...")) => {
                    self.reader.consume()?;
                    rest = self.parse_rest_binding_ident(BracketClose)?;
                    break;
                }
                token_matches!(@ident)
                | token_matches!(keyword!("await"))
                | token_matches!(keyword!("yield")) => {
                    let token = self.reader.consume()?;
                    comma_allowed = true;
                    bindings.push(ObjectBindingProp::Assign(token.try_into()?))
                }
                _ => return Err(Error::of(UnexpectedToken(self.reader.consume()?))),
            }
        }

        if self.reader.current()?.value == punct!(";") {
            self.reader.consume()?;
        }

        let span_end = self.reader.position();
        let span = (span_start, span_end);
        Ok(ObjectBinding::new(bindings, rest, span).into())
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

        let mut rest = None;
        let mut comma_delimiter = false;
        loop {
            match self.reader.current()? {
                token_matches!(punct!("]")) => {
                    self.reader.consume()?;
                    break;
                }
                token_matches!(punct!("{")) => {
                    let pat = self.parse_object_binding_pattern()?;
                    println!("PAT: {:?}", pat);
                    bindings.push(Some(pat));

                    comma_delimiter = true;
                }
                token_matches!(punct!(",")) => {
                    self.reader.consume()?;
                    if !comma_delimiter {
                        bindings.push(None);
                    }

                    comma_delimiter = false;
                }
                token_matches!(punct!("...")) => {
                    self.reader.consume()?;
                    rest = self.parse_rest_binding_ident(BraceClose)?;
                    break;
                }
                token_matches!(@ident)
                | token_matches!(keyword!("await"))
                | token_matches!(keyword!("yield")) => {
                    let token = self.reader.consume()?;
                    comma_delimiter = true;
                    bindings.push(Some(BindingPattern::Ident(token.try_into()?)))
                }
                _ => return Err(Error::of(UnexpectedToken(self.reader.consume()?))),
            }
        }

        let span_end = self.reader.position();
        let span = (span_start, span_end);

        Ok(ArrayBinding::new(bindings, rest, span).into())
    }

    /// Parses the BindingIdentifier goal symbol.
    /// This also consumes the expected end punctuator.
    ///
    /// Examples:
    /// var { ...rest } = a;
    ///          ^~~~~^
    /// var [ ...rest ] = a;
    ///          ^~~~~^
    fn parse_rest_binding_ident(&mut self, expected_end: Punct) -> Result<Option<Ident>> {
        let ident_token = self.reader.consume()?;
        if !token_matches!(ident_token, @ident)
            && !token_matches!(ident_token, keyword!("await"))
            && !token_matches!(ident_token, keyword!("yield"))
        {
            return Err(Error::of(UnexpectedToken(ident_token)));
        }

        let end_token = self.reader.consume()?;

        if let TokenValue::Punct(p) = end_token.value {
            if p == expected_end {
                return Ok(Some(ident_token.try_into().unwrap()));
            }
        }

        Err(Error::of(SyntaxError(
            "Rest element must be last element".to_owned(),
            ident_token.span,
        )))
    }
}
