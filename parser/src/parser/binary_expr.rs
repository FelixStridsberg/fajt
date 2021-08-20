use crate::error::Result;
use crate::Parser;
use fajt_ast::binary_op;
use fajt_ast::logical_op;
use fajt_ast::{BinaryOperator, Expr, ExprBinary, ExprLogical, LogicalOperator, Span};
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;

impl<'a, I> Parser<'a, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    /// Parses the `ShortCircuitExpression` goal symbol.
    pub(super) fn parse_short_circuit_expr(&mut self) -> Result<Expr> {
        self.parse_logical_expr(Self::parse_logical_or_expr, |token| {
            if token_matches!(token, punct!("??")) {
                Some(logical_op!("??"))
            } else {
                None
            }
        })
    }

    /// Parses the `LogicalORExpression` goal symbol.
    fn parse_logical_or_expr(&mut self) -> Result<Expr> {
        self.parse_logical_expr(Self::parse_logical_and_expr, |token| {
            if token_matches!(token, punct!("||")) {
                Some(logical_op!("||"))
            } else {
                None
            }
        })
    }

    /// Parses the `LogicalANDExpression` goal symbol.
    fn parse_logical_and_expr(&mut self) -> Result<Expr> {
        self.parse_logical_expr(Self::parse_bitwise_or_expr, |token| {
            if token_matches!(token, punct!("&&")) {
                Some(logical_op!("&&"))
            } else {
                None
            }
        })
    }

    /// Parses the `BitwiseORExpression` goal symbol.
    fn parse_bitwise_or_expr(&mut self) -> Result<Expr> {
        self.parse_binary_expr(Self::parse_bitwise_xor_expr, |token| {
            if token_matches!(token, punct!("|")) {
                Some(binary_op!("|"))
            } else {
                None
            }
        })
    }

    /// Parses the `BitwiseXORExpression` goal symbol.
    fn parse_bitwise_xor_expr(&mut self) -> Result<Expr> {
        self.parse_binary_expr(Self::parse_bitwise_and_expr, |token| {
            if token_matches!(token, punct!("^")) {
                Some(binary_op!("^"))
            } else {
                None
            }
        })
    }

    /// Parses the `BitwiseANDExpression` goal symbol.
    fn parse_bitwise_and_expr(&mut self) -> Result<Expr> {
        self.parse_binary_expr(Self::parse_equality_expr, |token| {
            if token_matches!(token, punct!("&")) {
                Some(binary_op!("&"))
            } else {
                None
            }
        })
    }

    /// Parses the `EqualityExpression` goal symbol.
    fn parse_equality_expr(&mut self) -> Result<Expr> {
        self.parse_binary_expr(Self::parse_relational_expr, |token| match token {
            token_matches!(punct!("==")) => Some(binary_op!("==")),
            token_matches!(punct!("!=")) => Some(binary_op!("!=")),
            token_matches!(punct!("===")) => Some(binary_op!("===")),
            token_matches!(punct!("!==")) => Some(binary_op!("!==")),
            _ => None,
        })
    }

    /// Parses the `RelationalExpression` goal symbol.
    fn parse_relational_expr(&mut self) -> Result<Expr> {
        let in_keyword_allowed = self.context.is_in;
        self.parse_binary_expr(Self::parse_shift_expr, |token| match token {
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
    fn parse_shift_expr(&mut self) -> Result<Expr> {
        self.parse_binary_expr(Self::parse_additive_expr, |token| match token {
            token_matches!(punct!("<<")) => Some(binary_op!("<<")),
            token_matches!(punct!(">>")) => Some(binary_op!(">>")),
            token_matches!(punct!(">>>")) => Some(binary_op!(">>>")),
            _ => None,
        })
    }

    /// Parses the `AdditiveExpression` goal symbol.
    fn parse_additive_expr(&mut self) -> Result<Expr> {
        self.parse_binary_expr(Self::parse_multiplicative_expr, |token| match token {
            token_matches!(punct!("+")) => Some(binary_op!("+")),
            token_matches!(punct!("-")) => Some(binary_op!("-")),
            _ => None,
        })
    }

    /// Parses the `MultiplicativeExpression` goal symbol.
    fn parse_multiplicative_expr(&mut self) -> Result<Expr> {
        self.parse_binary_expr(Self::parse_exponentiation_expr, |token| match token {
            token_matches!(punct!("*")) => Some(binary_op!("*")),
            token_matches!(punct!("/")) => Some(binary_op!("/")),
            token_matches!(punct!("%")) => Some(binary_op!("%")),
            _ => None,
        })
    }

    /// Parses the `ExponentiationExpression` goal symbol.
    fn parse_exponentiation_expr(&mut self) -> Result<Expr> {
        self.parse_binary_expr(Self::parse_unary_expr, |token| {
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
    fn parse_binary_expr<F>(
        &mut self,
        next: fn(&mut Self) -> Result<Expr>,
        map_operator: F,
    ) -> Result<Expr>
    where
        F: Fn(&Token) -> Option<BinaryOperator>,
    {
        self.parse_recursive_binary_expr(next, map_operator, |span, left, right, operator| {
            ExprBinary {
                span,
                left: left.into(),
                right: right.into(),
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
    fn parse_logical_expr(
        &mut self,
        next: fn(&mut Self) -> Result<Expr>,
        map_operator: fn(&Token) -> Option<LogicalOperator>,
    ) -> Result<Expr> {
        self.parse_recursive_binary_expr(next, map_operator, |span, left, right, operator| {
            ExprLogical {
                span,
                left: left.into(),
                right: right.into(),
                operator,
            }
            .into()
        })
    }

    #[inline]
    fn parse_recursive_binary_expr<T, F>(
        &mut self,
        next: fn(&mut Self) -> Result<Expr>,
        map_operator: F,
        create_expr: fn(span: Span, left: Expr, right: Expr, operator: T) -> Expr,
    ) -> Result<Expr>
    where
        F: Fn(&Token) -> Option<T>,
    {
        let span_start = self.position();
        let mut expr = next(self)?;
        loop {
            let operator = self.current().map(|t| map_operator(t));

            if let Ok(Some(operator)) = operator {
                self.consume()?;
                let right = next(self)?;
                let span = self.span_from(span_start);

                expr = create_expr(span, expr, right, operator);
            } else {
                break;
            }
        }

        Ok(expr)
    }
}
