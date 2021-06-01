use crate::ast::Expression::IdentifierReference;
use crate::ast::{Expression, Literal, ThisExpression, UnaryExpression};
use crate::error::Result;
use crate::Parser;

use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token_matches;

impl Parser<'_, '_> {
    /// Parses the `Expression` goal symbol.
    pub(super) fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_assignment_expression()
        // TODO comma separated expressions: Expression::Sequence(Vec<Expression>)?
    }

    /// Parses the `AssignmentExpression` goal symbol.
    pub(super) fn parse_assignment_expression(&mut self) -> Result<Expression> {
        self.parse_conditional_expression()
        // TODO YieldExpression
        // TODO ArrowFunction
        // TODO AsyncArrowFunction
        // TODO LeftHandSideExpression
    }

    /// Parses the `ConditionalExpression` goal symbol.
    fn parse_conditional_expression(&mut self) -> Result<Expression> {
        self.parse_short_circuit_expression()
        // TODO ShortCircuitExpression ? AssignmentExpression : AssignmentExpression
    }

    /// Parses the `UnaryExpression` goal symbol.
    pub(super) fn parse_unary_expression(&mut self) -> Result<Expression> {
        let operator = match self.reader.current()? {
            token_matches!(punct!("+")) => Some(unary_op!("+")),
            _ => None,
        };

        if let Some(operator) = operator {
            let span_start = self.position();
            self.reader.consume()?;
            let argument = self.parse_unary_expression()?;
            let span = self.span_from(span_start);
            Ok(Expression::UnaryExpression(Box::new(UnaryExpression {
                span,
                operator,
                argument,
            })))
        } else {
            self.parse_update_expression()
        }

        // TODO delete UnaryExpression
        // TODO typeof UnaryExpression
        // TODO - UnaryExpression
        // TODO ~ UnaryExpression
        // TODO ! UnaryExpression
        // TODO AwaitExpression (await UnaryExpression)
    }

    /// Parses the `UpdateExpression` goal symbol.
    fn parse_update_expression(&mut self) -> Result<Expression> {
        self.parse_left_hand_side_expression()
        // TODO LeftHandSideExpression [no LineTerminator] ++
        // TODO LeftHandSideExpression [no LineTerminator] --
        // TODO ++ UnaryExpression
        // TODO -- UnaryExpression
    }

    /// Parses the `LeftHandSideExpression` goal symbol.
    fn parse_left_hand_side_expression(&mut self) -> Result<Expression> {
        self.parse_new_expression()
        // TODO CallExpression
        // TODO OptionalExpression
    }

    /// Parses the `NewExpression` goal symbol.
    fn parse_new_expression(&mut self) -> Result<Expression> {
        self.parse_member_expression()
        // TODO new NewExpression
    }

    /// Parses the `MemberExpression` goal symbol.
    fn parse_member_expression(&mut self) -> Result<Expression> {
        self.parse_primary_expression()
        // TODO MemberExpression [ Expression ]
        // TODO MemberExpression . IdentifierName
        // TODO MemberExpression TemplateLiteral
        // TODO SuperProperty
        // TODO MetaProperty
        // TODO new MemberExpression Arguments
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
    fn parse_this_expression(&mut self) -> Result<Expression> {
        let token = self.reader.consume()?;
        debug_assert!(token_matches!(token, keyword!("this")));

        Ok(ThisExpression::new(token.span).into())
    }

    /// Parses the `IdentifierReference` goal symbol.
    fn parse_identifier_reference(&mut self) -> Result<Expression> {
        let ident = self.parse_identifier()?;

        // Consume potential trailing semi colon TODO how to handle these in general?
        if token_matches!(self.reader.current(), ok: punct!(";")) {
            self.reader.consume()?;
        }

        Ok(IdentifierReference(Box::new(ident.into())))
    }

    /// Parses the `Initializer` goal symbol.
    pub(super) fn parse_initializer(&mut self) -> Result<Expression> {
        self.reader.consume()?; // Skip =
        self.parse_assignment_expression()
    }
}
