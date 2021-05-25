use crate::ast::Expression::IdentifierReference;
use crate::ast::{Expression, Ident, Literal, ThisExpression};
use crate::error::Result;
use crate::Parser;

use fajt_lexer::keyword;
use fajt_lexer::punct;

use fajt_lexer::token_matches;

impl Parser<'_, '_> {
    pub(super) fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_assignment_expression()
    }

    pub(super) fn parse_assignment_expression(&mut self) -> Result<Expression> {
        // TODO other than primary expressions
        self.parse_primary_expression()
    }

    /// Parses the `PrimaryExpression` goal symbol.
    fn parse_primary_expression(&mut self) -> Result<Expression> {
        Ok(match self.reader.current()? {
            token_matches!(keyword!("this")) => self.parse_this_expression()?,
            token_matches!(keyword!("yield")) => unimplemented!(),
            token_matches!(keyword!("await")) => unimplemented!(),
            token_matches!(keyword!("null")) => self.consume_literal(Literal::Null)?,
            token_matches!(keyword!("true")) => self.consume_literal(Literal::Boolean(true))?,
            token_matches!(keyword!("false")) => self.consume_literal(Literal::Boolean(false))?,
            token_matches!(@literal) => self.parse_literal()?,
            token_matches!(punct!("[")) => self.parse_array_literal()?,
            token_matches!(punct!("{")) => self.parse_object_literal()?,
            // TODO FunctionExpression
            // TODO ClassExpression
            // TODO GeneratorExpression
            // TODO AsyncFunctionExpression
            // TODO AsyncGeneratorExpression
            // TODO RegularExpressionLiteral
            // TODO TemplateLiteral
            // TODO CoverParenthesizedExpressionAndArrowParameterList
            token_matches!(@ident) => self.parse_identifier_reference()?,
            r => unimplemented!("TOKEN: {:?}", r),
        })
    }

    /// Parses the `this` expression which is part of the `PrimaryExpression` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// this
    /// ^~~^
    /// ```
    fn parse_this_expression(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("this")));

        Ok(ThisExpression::new(token.span).into())
    }

    /// Parses the `IdentifierReference` goal symbol, which is part of the `PrimaryExpression`.
    ///
    /// Example:
    /// ```no_rust
    /// var foo = bar;
    ///           ^~^
    /// ```
    fn parse_identifier_reference(&mut self) -> Result<Expression> {
        let ident: Ident = self.parse_identifier()?;

        // Consume potential trailing semi colon TODO how to handle these in general?
        if self.reader.current()?.value == punct!(";") {
            self.reader.consume()?;
        }

        Ok(IdentifierReference(ident.into()))
    }

    /// Parses the `Initializer` goal symbol.
    ///
    /// Example:
    /// ```no_rust
    /// var a = 1 + 2, b = 100;
    ///         ^~~~^      ^~^
    /// ```
    pub(super) fn parse_initializer(&mut self) -> Result<Expression> {
        self.reader.consume()?; // Skip =
        self.parse_assignment_expression()
    }
}