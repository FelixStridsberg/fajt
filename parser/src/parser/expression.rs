use crate::ast::Expression::IdentifierReference;
use crate::ast::{
    BinaryExpression, BinaryOperator, Expression, Ident, Literal, LogicalExpression,
    LogicalOperator, ThisExpression,
};
use crate::error::Result;
use crate::Parser;

use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::{Span, Token};
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

    /// Parses the `ShortCircuitExpression` goal symbol.
    fn parse_short_circuit_expression(&mut self) -> Result<Expression> {
        self.parse_logical_expression(Self::parse_logical_or_expression, |token| {
            if token_matches!(token, punct!("??")) {
                Some(logical_op!("??"))
            } else {
                None
            }
        })
    }

    /// Parses the `LogicalORExpression` goal symbol.
    fn parse_logical_or_expression(&mut self) -> Result<Expression> {
        self.parse_logical_expression(Self::parse_logical_and_expression, |token| {
            if token_matches!(token, punct!("||")) {
                Some(logical_op!("||"))
            } else {
                None
            }
        })
    }

    /// Parses the `LogicalANDExpression` goal symbol.
    fn parse_logical_and_expression(&mut self) -> Result<Expression> {
        self.parse_logical_expression(Self::parse_bitwise_or_expression, |token| {
            if token_matches!(token, punct!("&&")) {
                Some(logical_op!("&&"))
            } else {
                None
            }
        })
    }

    /// Parses the `BitwiseORExpression` goal symbol.
    fn parse_bitwise_or_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expression(Self::parse_bitwise_xor_expression, |token| {
            if token_matches!(token, punct!("|")) {
                Some(binary_op!("|"))
            } else {
                None
            }
        })
    }

    /// Parses the `BitwiseXORExpression` goal symbol.
    fn parse_bitwise_xor_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expression(Self::parse_bitwise_and_expression, |token| {
            if token_matches!(token, punct!("^")) {
                Some(binary_op!("^"))
            } else {
                None
            }
        })
    }

    /// Parses the `BitwiseANDExpression` goal symbol.
    fn parse_bitwise_and_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expression(Self::parse_equality_expression, |token| {
            if token_matches!(token, punct!("&")) {
                Some(binary_op!("&"))
            } else {
                None
            }
        })
    }

    /// Parses the `EqualityExpression` goal symbol.
    fn parse_equality_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expression(Self::parse_relational_expression, |token| match token {
            token_matches!(punct!("==")) => Some(binary_op!("==")),
            token_matches!(punct!("!=")) => Some(binary_op!("!=")),
            token_matches!(punct!("===")) => Some(binary_op!("===")),
            token_matches!(punct!("!==")) => Some(binary_op!("!==")),
            _ => None,
        })
    }

    /// Parses the `RelationalExpression` goal symbol.
    fn parse_relational_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expression(Self::parse_shift_expression, |token| match token {
            token_matches!(punct!("<")) => Some(binary_op!("<")),
            token_matches!(punct!(">")) => Some(binary_op!(">")),
            token_matches!(punct!("<=")) => Some(binary_op!("<=")),
            token_matches!(punct!(">=")) => Some(binary_op!(">=")),
            token_matches!(keyword!("instanceof")) => Some(binary_op!("instanceof")),
            token_matches!(keyword!("in")) => Some(binary_op!("in")),
            _ => None,
        })
    }

    /// Parses the `ShiftExpression` goal symbol.
    fn parse_shift_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expression(Self::parse_additive_expression, |token| match token {
            token_matches!(punct!("<<")) => Some(binary_op!("<<")),
            token_matches!(punct!(">>")) => Some(binary_op!(">>")),
            token_matches!(punct!(">>>")) => Some(binary_op!(">>>")),
            _ => None,
        })
    }

    /// Parses the `AdditiveExpression` goal symbol.
    fn parse_additive_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expression(Self::parse_multiplicative_expression, |token| match token {
            token_matches!(punct!("+")) => Some(binary_op!("+")),
            token_matches!(punct!("-")) => Some(binary_op!("-")),
            _ => None,
        })
    }

    /// Parses the `MultiplicativeExpression` goal symbol.
    fn parse_multiplicative_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expression(Self::parse_exponentiation_expression, |token| match token {
            token_matches!(punct!("*")) => Some(binary_op!("*")),
            token_matches!(punct!("/")) => Some(binary_op!("/")),
            token_matches!(punct!("%")) => Some(binary_op!("%")),
            _ => None,
        })
    }

    /// Parses the `ExponentiationExpression` goal symbol.
    fn parse_exponentiation_expression(&mut self) -> Result<Expression> {
        self.parse_binary_expression(Self::parse_unary_expression, |token| {
            if token_matches!(token, punct!("**")) {
                Some(binary_op!("**"))
            } else {
                None
            }
        })
    }

    /// All binary expressions are parsed the same way, they are broken up into multiple goal
    /// symbols for precedence. This is the common parse method for all of them.
    ///
    /// `next` is a method for retrieving the result of the _next_ goal symbol (i.e. right hand).
    /// `map_operator` is a function for mapping a token to a binary operator.
    #[inline]
    fn parse_binary_expression(
        &mut self,
        next: fn(&mut Self) -> Result<Expression>,
        map_operator: fn(&Token) -> Option<BinaryOperator>,
    ) -> Result<Expression> {
        self.parse_recursive_binary_expression(next, map_operator, |span, left, right, operator| {
            Expression::BinaryExpression(Box::new(BinaryExpression {
                span,
                left,
                right,
                operator,
            }))
        })
    }

    /// All binary expressions are parsed the same way, they are broken up into multiple goal
    /// symbols for precedence. This is the common parse method for all of them.
    ///
    /// `next` is a method for retrieving the result of the _next_ goal symbol (i.e. right hand).
    /// `map_operator` is a function for mapping a token to a binary operator.
    #[inline]
    fn parse_logical_expression(
        &mut self,
        next: fn(&mut Self) -> Result<Expression>,
        map_operator: fn(&Token) -> Option<LogicalOperator>,
    ) -> Result<Expression> {
        self.parse_recursive_binary_expression(next, map_operator, |span, left, right, operator| {
            Expression::LogicalExpression(Box::new(LogicalExpression {
                span,
                left,
                right,
                operator,
            }))
        })
    }

    #[inline]
    fn parse_recursive_binary_expression<T>(
        &mut self,
        next: fn(&mut Self) -> Result<Expression>,
        map_operator: fn(&Token) -> Option<T>,
        create_expression: fn(
            span: Span,
            left: Expression,
            right: Expression,
            operator: T,
        ) -> Expression,
    ) -> Result<Expression> {
        let span_start = self.position();
        let mut expression = next(self)?;
        loop {
            let operator = self.reader.current().map(|t| map_operator(t));

            if let Ok(Some(operator)) = operator {
                self.reader.consume()?;
                let right = next(self)?;
                let span = self.span_from(span_start);

                expression = create_expression(span, expression, right, operator);
            } else {
                break;
            }
        }

        Ok(expression)
    }

    /// Parses the `UnaryExpression` goal symbol.
    fn parse_unary_expression(&mut self) -> Result<Expression> {
        self.parse_update_expression()
        // TODO delete UnaryExpression
        // TODO typeof UnaryExpression
        // TODO + UnaryExpression
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
        if token_matches!(self.reader.current(), ok: punct!(";")) {
            self.reader.consume()?;
        }

        Ok(IdentifierReference(Box::new(ident.into())))
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
