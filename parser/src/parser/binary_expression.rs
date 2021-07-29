use crate::ast::{BinaryExpression, BinaryOperator, Expr, LogicalExpression, LogicalOperator};
use crate::error::Result;
use crate::Parser;

use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::{Span, Token};
use fajt_lexer::token_matches;

impl<'a, I> Parser<'a, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    /// Parses the `ShortCircuitExpression` goal symbol.
    pub(super) fn parse_short_circuit_expression(&mut self) -> Result<Expr> {
        self.parse_logical_expression(Self::parse_logical_or_expression, |token| {
            if token_matches!(token, punct!("??")) {
                Some(logical_op!("??"))
            } else {
                None
            }
        })
    }

    /// Parses the `LogicalORExpression` goal symbol.
    fn parse_logical_or_expression(&mut self) -> Result<Expr> {
        self.parse_logical_expression(Self::parse_logical_and_expression, |token| {
            if token_matches!(token, punct!("||")) {
                Some(logical_op!("||"))
            } else {
                None
            }
        })
    }

    /// Parses the `LogicalANDExpression` goal symbol.
    fn parse_logical_and_expression(&mut self) -> Result<Expr> {
        self.parse_logical_expression(Self::parse_bitwise_or_expression, |token| {
            if token_matches!(token, punct!("&&")) {
                Some(logical_op!("&&"))
            } else {
                None
            }
        })
    }

    /// Parses the `BitwiseORExpression` goal symbol.
    fn parse_bitwise_or_expression(&mut self) -> Result<Expr> {
        self.parse_binary_expression(Self::parse_bitwise_xor_expression, |token| {
            if token_matches!(token, punct!("|")) {
                Some(binary_op!("|"))
            } else {
                None
            }
        })
    }

    /// Parses the `BitwiseXORExpression` goal symbol.
    fn parse_bitwise_xor_expression(&mut self) -> Result<Expr> {
        self.parse_binary_expression(Self::parse_bitwise_and_expression, |token| {
            if token_matches!(token, punct!("^")) {
                Some(binary_op!("^"))
            } else {
                None
            }
        })
    }

    /// Parses the `BitwiseANDExpression` goal symbol.
    fn parse_bitwise_and_expression(&mut self) -> Result<Expr> {
        self.parse_binary_expression(Self::parse_equality_expression, |token| {
            if token_matches!(token, punct!("&")) {
                Some(binary_op!("&"))
            } else {
                None
            }
        })
    }

    /// Parses the `EqualityExpression` goal symbol.
    fn parse_equality_expression(&mut self) -> Result<Expr> {
        self.parse_binary_expression(Self::parse_relational_expression, |token| match token {
            token_matches!(punct!("==")) => Some(binary_op!("==")),
            token_matches!(punct!("!=")) => Some(binary_op!("!=")),
            token_matches!(punct!("===")) => Some(binary_op!("===")),
            token_matches!(punct!("!==")) => Some(binary_op!("!==")),
            _ => None,
        })
    }

    /// Parses the `RelationalExpression` goal symbol.
    fn parse_relational_expression(&mut self) -> Result<Expr> {
        let in_keyword_allowed = self.context.is_in;
        self.parse_binary_expression(Self::parse_shift_expression, |token| match token {
            token_matches!(punct!("<")) => Some(binary_op!("<")),
            token_matches!(punct!(">")) => Some(binary_op!(">")),
            token_matches!(punct!("<=")) => Some(binary_op!("<=")),
            token_matches!(punct!(">=")) => Some(binary_op!(">=")),
            token_matches!(keyword!("instanceof")) => Some(binary_op!("instanceof")),
            token_matches!(keyword!("in")) if in_keyword_allowed => Some(binary_op!("in")),
            _ => None,
        })
    }

    /// Parses the `ShiftExpression` goal symbol.
    fn parse_shift_expression(&mut self) -> Result<Expr> {
        self.parse_binary_expression(Self::parse_additive_expression, |token| match token {
            token_matches!(punct!("<<")) => Some(binary_op!("<<")),
            token_matches!(punct!(">>")) => Some(binary_op!(">>")),
            token_matches!(punct!(">>>")) => Some(binary_op!(">>>")),
            _ => None,
        })
    }

    /// Parses the `AdditiveExpression` goal symbol.
    fn parse_additive_expression(&mut self) -> Result<Expr> {
        self.parse_binary_expression(Self::parse_multiplicative_expression, |token| match token {
            token_matches!(punct!("+")) => Some(binary_op!("+")),
            token_matches!(punct!("-")) => Some(binary_op!("-")),
            _ => None,
        })
    }

    /// Parses the `MultiplicativeExpression` goal symbol.
    fn parse_multiplicative_expression(&mut self) -> Result<Expr> {
        self.parse_binary_expression(Self::parse_exponentiation_expression, |token| match token {
            token_matches!(punct!("*")) => Some(binary_op!("*")),
            token_matches!(punct!("/")) => Some(binary_op!("/")),
            token_matches!(punct!("%")) => Some(binary_op!("%")),
            _ => None,
        })
    }

    /// Parses the `ExponentiationExpression` goal symbol.
    fn parse_exponentiation_expression(&mut self) -> Result<Expr> {
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
    fn parse_binary_expression<F>(
        &mut self,
        next: fn(&mut Self) -> Result<Expr>,
        map_operator: F,
    ) -> Result<Expr>
    where
        F: Fn(&Token) -> Option<BinaryOperator>,
    {
        self.parse_recursive_binary_expression(next, map_operator, |span, left, right, operator| {
            BinaryExpression {
                span,
                left,
                right,
                operator,
            }
            .into()
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
        next: fn(&mut Self) -> Result<Expr>,
        map_operator: fn(&Token) -> Option<LogicalOperator>,
    ) -> Result<Expr> {
        self.parse_recursive_binary_expression(next, map_operator, |span, left, right, operator| {
            LogicalExpression {
                span,
                left,
                right,
                operator,
            }
            .into()
        })
    }

    #[inline]
    fn parse_recursive_binary_expression<T, F>(
        &mut self,
        next: fn(&mut Self) -> Result<Expr>,
        map_operator: F,
        create_expression: fn(span: Span, left: Expr, right: Expr, operator: T) -> Expr,
    ) -> Result<Expr>
    where
        F: Fn(&Token) -> Option<T>,
    {
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
}
