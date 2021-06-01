use crate::ast::Expression::IdentifierReference;
use crate::ast::{
    Expression, Literal, ThisExpression, UnaryExpression, UpdateExpression, YieldExpression,
};
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
        match self.reader.current() {
            token_matches!(ok: keyword!("yield")) => self.parse_yield_expression(),
            _ => self.parse_conditional_expression(),
        }

        // TODO ArrowFunction
        // TODO AsyncArrowFunction
        // TODO LeftHandSideExpression
    }

    // Parses the `YieldExpression` goal symbol.
    fn parse_yield_expression(&mut self) -> Result<Expression> {
        let span_start = self.position();
        let yield_token = self.reader.consume()?;
        debug_assert!(token_matches!(yield_token, keyword!("yield")));

        if self.reader.is_end() {
            let span = self.span_from(span_start);
            return Ok(Expression::YieldExpression(Box::new(YieldExpression {
                span,
                argument: None,
                delegate: false,
            })));
        }

        let next_token = self.reader.current()?;
        let delegate = token_matches!(next_token, punct!("*"));
        if delegate {
            self.reader.consume()?;
        }

        let argument = self.parse_assignment_expression()?;
        let span = self.span_from(span_start);
        Ok(Expression::YieldExpression(Box::new(YieldExpression {
            span,
            argument: Some(argument),
            delegate,
        })))
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
            token_matches!(punct!("-")) => Some(unary_op!("-")),
            token_matches!(punct!("~")) => Some(unary_op!("~")),
            token_matches!(punct!("!")) => Some(unary_op!("!")),
            token_matches!(keyword!("delete")) => Some(unary_op!("delete")),
            token_matches!(keyword!("void")) => Some(unary_op!("void")),
            token_matches!(keyword!("typeof")) => Some(unary_op!("typeof")),
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

        // TODO AwaitExpression (await UnaryExpression)
    }

    /// Parses the `UpdateExpression` goal symbol.
    fn parse_update_expression(&mut self) -> Result<Expression> {
        let prefix_operator = match self.reader.current()? {
            token_matches!(punct!("++")) => Some(update_op!("++")),
            token_matches!(punct!("--")) => Some(update_op!("--")),
            _ => None,
        };

        if let Some(operator) = prefix_operator {
            let span_start = self.position();
            self.reader.consume()?;
            let argument = self.parse_unary_expression()?;
            let span = self.span_from(span_start);
            return Ok(Expression::UpdateExpression(Box::new(UpdateExpression {
                span,
                operator,
                prefix: true,
                argument,
            })));
        }

        let span_start = self.position();
        let argument = self.parse_left_hand_side_expression()?;
        let suffix_operator = match self.reader.current() {
            token_matches!(ok: punct!("++")) => Some(update_op!("++")),
            token_matches!(ok: punct!("--")) => Some(update_op!("--")),
            _ => None,
        };

        if let Some(operator) = suffix_operator {
            if self.reader.current()?.first_on_line {
                return Ok(argument);
            }

            self.reader.consume()?;
            let span = self.span_from(span_start);
            Ok(Expression::UpdateExpression(Box::new(UpdateExpression {
                span,
                operator,
                prefix: false,
                argument,
            })))
        } else {
            Ok(argument)
        }
    }

    /// Parses the `LeftHandSideExpression` goal symbol.
    fn parse_left_hand_side_expression(&mut self) -> Result<Expression> {
        self.parse_new_expression()
        // TODO CallExpression
        // TODO OptionalExpression
    }

    /// Parses the `NewExpression` goal symbol.
    fn parse_new_expression(&mut self) -> Result<Expression> {
        if token_matches!(self.reader.current(), ok: keyword!("new")) {
            todo!("NewExpression")
        } else {
            self.parse_member_expression()
        }
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
