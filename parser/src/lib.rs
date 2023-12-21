extern crate core;
extern crate serde;

#[macro_use]
pub mod error;
mod assignment_pattern;
mod binary_expr;
mod binding;
mod class;
mod cover;
mod expr;
mod function;
mod iteration;
mod literal;
mod member_access;
mod method;
mod module;
mod static_semantics;
mod stmt;
mod variable;

use crate::error::{Error, Result};
use crate::static_semantics::DirectivePrologueSemantics;
use fajt_ast::{
    Expr, ExprLiteral, Ident, LitString, Literal, Program, PropertyName, SourceType, Span, Stmt,
    StmtExpr, StmtList,
};
use fajt_common::io::{PeekRead, PeekReader, ReReadWithState};
use fajt_lexer::error::ErrorKind as LexerErrorKind;
use fajt_lexer::token::{KeywordContext, Token, TokenValue};
use fajt_lexer::{punct, Lexer};
use fajt_lexer::{token_matches, LexerState};

/// Similar trait to bool.then, but handles closures returning `Result`.
pub trait ThenTry {
    fn then_try<T, F>(self, f: F) -> Result<Option<T>>
    where
        F: FnOnce() -> Result<T>;
}

impl ThenTry for bool {
    fn then_try<T, F>(self, f: F) -> Result<Option<T>>
    where
        F: FnOnce() -> Result<T>,
    {
        if self {
            f().map(Some)
        } else {
            Ok(None)
        }
    }
}

pub fn parse_script(source: &str) -> Result<Program> {
    parse::<Program>(source, SourceType::Script)
}

pub fn parse_module(source: &str) -> Result<Program> {
    parse::<Program>(source, SourceType::Module)
}

/// Parse source into `Program` when type of source is known.
pub fn parse<T>(source: &str, source_type: SourceType) -> Result<T>
where
    T: Parse,
{
    let mut lexer = Lexer::new(source).unwrap();
    if source_type == SourceType::Module {
        lexer.set_state(LexerState::default().with_html_comments_allowed(false));
    };

    let mut reader = PeekReader::new(lexer).unwrap();
    Parser::parse::<T>(&mut reader, source_type)
}

/// Context of the parser.
#[derive(Clone, Default)]
pub struct Context {
    /// `Await` production parameter.
    is_await: bool,

    /// `Yield` production parameter.
    is_yield: bool,

    /// `In` production parameter.
    is_in: bool,

    /// `Strict` production parameter.
    is_strict: bool,

    /// `Default` production parameter.
    is_default: bool,

    /// `Return` production parameter.
    is_return: bool,

    /// `true` if we are inside a method and not a function.
    in_method: bool,

    /// `true` if we are in a context where `super()`-call is allowed.
    super_call_allowed: bool,

    /// `true` if static methods are allowed.
    static_method_allowed: bool,

    /// `true` if we are currently parsing assignment pattern
    is_inside_assignment_expr: bool,
}

macro_rules! modifier {
    ($fn_name:ident:$field_name:ident) => {
        #[doc = concat!("Returns a new context with specified `", stringify!($field_name), "` value.")]
        pub fn $fn_name(&self, $field_name: bool) -> Self {
            Context {
                $field_name,
                ..(*self)
            }
        }
    };
}

impl Context {
    modifier!(with_await: is_await);
    modifier!(with_yield: is_yield);
    modifier!(with_in: is_in);
    modifier!(with_strict: is_strict);
    modifier!(with_default: is_default);
    modifier!(with_return: is_return);

    modifier!(inside_assignment_expr: is_inside_assignment_expr);
    modifier!(with_in_method: in_method);
    modifier!(with_super_call_allowed: super_call_allowed);
    modifier!(with_static_method_allowed: static_method_allowed);

    /// Resets all production parameters.
    fn reset_parameters(&self) -> Self {
        Context {
            in_method: self.in_method,
            super_call_allowed: self.super_call_allowed,
            static_method_allowed: self.static_method_allowed,
            ..Context::default()
        }
    }

    fn keyword_context(&self) -> KeywordContext {
        let mut keyword_context = KeywordContext::empty();
        if self.is_await {
            keyword_context |= KeywordContext::AWAIT;
        }

        if self.is_yield {
            keyword_context |= KeywordContext::YIELD;
        }

        if self.is_strict {
            keyword_context |= KeywordContext::STRICT;
        }

        keyword_context
    }
}

pub trait Parse: Sized {
    fn parse<I>(parser: &mut Parser<I>) -> Result<Self>
    where
        I: PeekRead<Token, Error = fajt_lexer::error::Error>,
        I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>;
}

impl Parse for Expr {
    fn parse<I>(parser: &mut Parser<I>) -> Result<Self>
    where
        I: PeekRead<Token, Error = fajt_lexer::error::Error>,
        I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
    {
        parser
            .with_context(Context::default().with_in(true))
            .parse_expr()
    }
}

impl Parse for Stmt {
    fn parse<I>(parser: &mut Parser<I>) -> Result<Self>
    where
        I: PeekRead<Token, Error = fajt_lexer::error::Error>,
        I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
    {
        parser.parse_declaration_or_statement()
    }
}

impl Parse for Program {
    fn parse<I>(parser: &mut Parser<I>) -> Result<Self>
    where
        I: PeekRead<Token, Error = fajt_lexer::error::Error>,
        I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
    {
        let span_start = parser.position();

        let directives = parser.parse_directive_prologue()?;
        let strict_mode = directives.as_slice().contains_strict();

        let body = if strict_mode {
            parser
                .with_context(parser.context.with_strict(true))
                .parse_all_stmts()?
        } else {
            parser.parse_all_stmts()?
        };

        let span = parser.span_from(span_start);
        let stmt_list = StmtList {
            span,
            directives,
            body,
        };

        Ok(Program::new(parser.source_type, stmt_list))
    }
}

pub struct Parser<'a, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    context: Context,
    reader: &'a mut PeekReader<Token, I>,
    source_type: SourceType,
}

impl<'a, I> Parser<'a, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    pub fn new(reader: &'a mut PeekReader<Token, I>, source_type: SourceType) -> Result<Self> {
        Ok(Parser {
            context: Context::default(),
            reader,
            source_type,
        })
    }

    pub fn parse<T>(reader: &'a mut PeekReader<Token, I>, source_type: SourceType) -> Result<T>
    where
        T: Parse,
    {
        let mut parser = Parser::new(reader, source_type)?;
        T::parse(&mut parser)
    }

    fn current(&self) -> Result<&Token> {
        Ok(self.reader.current()?)
    }

    fn consume(&mut self) -> Result<Token> {
        Ok(self.reader.consume()?)
    }

    fn peek(&self) -> Option<&Token> {
        self.reader.peek().ok()
    }

    fn is_end(&self) -> bool {
        self.reader
            .current()
            .err()
            .map_or(false, |error| error.kind() == &LexerErrorKind::EndOfStream)
    }

    fn position(&self) -> usize {
        self.current()
            .map(|t| t.span.start)
            .unwrap_or_else(|_| self.reader.position())
    }

    /// Returns a `Span` that ends at current position and starts from `start`.
    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.reader.position())
    }

    /// Returns a new parser at current position with different context.
    pub fn with_context(&mut self, context: Context) -> Parser<'_, I> {
        Parser {
            context,
            reader: self.reader,
            source_type: self.source_type,
        }
    }

    /// Returns `true` if current token matches `value`.
    fn current_matches(&self, value: &TokenValue) -> bool {
        if let Ok(token) = self.current() {
            &token.value == value
        } else {
            false
        }
    }

    /// Returns `true` if current token is a string literal.
    fn current_matches_string_literal(&self) -> bool {
        matches!(
            self.current(),
            Ok(Token {
                value: TokenValue::Literal(Literal::String(_)),
                ..
            })
        )
    }

    /// Returns `true` if current token is an identifier or string literal that
    /// matches `value`.
    fn current_matches_identifier_or_literal(&self, value: &str) -> bool {
        match self.current() {
            Ok(Token {
                value: TokenValue::Identifier(identifier),
                ..
            }) => value == identifier,
            Ok(Token {
                value: TokenValue::Literal(Literal::String(LitString { value: string, .. })),
                ..
            }) => value == string,
            _ => false,
        }
    }

    /// Returns `true` if next token matches `value`.
    fn peek_matches(&self, value: &TokenValue) -> bool {
        if let Some(token) = self.peek() {
            &token.value == value
        } else {
            false
        }
    }

    /// Consumes current token. Returns error if consumed token do not match `expected`.
    fn consume_assert(&mut self, expected: &'static TokenValue) -> Result<Token> {
        let token = self.consume()?;
        if &token.value != expected {
            return Err(Error::expected_other_token(token, expected));
        }
        Ok(token)
    }

    /// Consumes current token if it matches `value`. Returns `true` if current token was consumed.
    fn maybe_consume(&mut self, value: &TokenValue) -> Result<bool> {
        if self.current_matches(value) {
            self.consume()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns `true` if current token is followed by new line.
    fn followed_by_new_line(&self) -> bool {
        self.peek().map_or(false, |t| t.first_on_line)
    }

    /// Returns `true` if current token is followed by new line.
    fn first_on_line(&self) -> bool {
        self.current().map_or(false, |t| t.first_on_line)
    }

    /// Returns `true` if next token could be parsed to a valid identifier.
    fn peek_is_identifier(&self) -> bool {
        is_identifier(self.peek(), self.context.keyword_context())
    }

    /// Returns `true` if current token could be parsed to a valid identifier.
    fn is_identifier(&self) -> bool {
        is_identifier(self.current().ok(), self.context.keyword_context())
    }

    /// Returns `true` if current token could be parsed to a valid identifier name.
    fn is_identifier_name(&self) -> bool {
        matches!(
            self.current(),
            Ok(Token {
                value: TokenValue::Identifier(_) | TokenValue::Keyword(_),
                ..
            })
        )
    }

    /// Tries to parse an identifier from current token, either directly from a `Identifier` token
    /// or from `Keyword` if the keyword is allowed in current context.
    fn parse_identifier(&mut self) -> Result<Ident> {
        let token = self.consume()?;
        Ok(match token.value {
            TokenValue::Identifier(s) => Ident::new(s, token.span),
            TokenValue::Keyword(keyword) => {
                if keyword.is_allowed_as_identifier(self.context.keyword_context()) {
                    Ident::new(keyword.to_string(), token.span)
                } else {
                    return Err(Error::forbidden_identifier(keyword.to_string(), token.span));
                }
            }
            _ => return Err(Error::expected_ident(token)),
        })
    }

    /// Parses the `IdentifierName` production, this do not respect the reserved words.
    fn parse_identifier_name(&mut self) -> Result<Ident> {
        let token = self.consume()?;
        Ok(match token.value {
            TokenValue::Identifier(s) => Ident::new(s, token.span),
            TokenValue::Keyword(keyword) => Ident::new(keyword.to_string(), token.span),
            _ => return Err(Error::expected_ident(token)),
        })
    }

    /// Parses the `PropertyName` production.
    fn parse_property_name(&mut self) -> Result<PropertyName> {
        match self.current()? {
            token_matches!(@literal) => {
                let token = self.consume()?;
                match token.value {
                    TokenValue::Literal(Literal::String(string)) => {
                        Ok(PropertyName::String(string))
                    }
                    TokenValue::Literal(Literal::Number(number)) => {
                        Ok(PropertyName::Number(number))
                    }
                    _ => Err(Error::unexpected_token(token)),
                }
            }
            token_matches!(punct!("[")) => {
                self.consume()?;
                let expr = self.parse_assignment_expr()?;
                self.consume_assert(&punct!("]"))?;
                Ok(PropertyName::Computed(expr.into()))
            }
            _ if self.is_identifier_name() => {
                Ok(PropertyName::Ident(self.parse_identifier_name()?))
            }
            _ => Err(Error::unexpected_token(self.consume()?)),
        }
    }

    fn parse_directive_prologue(&mut self) -> Result<Vec<LitString>> {
        let mut directives = Vec::new();

        loop {
            if !self.current_matches_string_literal() {
                break;
            }

            let stmt_start_token = self.current()?.clone();
            let string_literal = match self.parse_declaration_or_statement()? {
                Stmt::Expr(StmtExpr { expr, .. }) => match *expr {
                    Expr::Literal(ExprLiteral {
                        literal: Literal::String(string),
                        ..
                    }) => Some(string),
                    _ => None,
                },
                _ => None,
            };

            if let Some(string) = string_literal {
                directives.push(string);
            } else {
                self.reader.rewind_to(&stmt_start_token)?;
                break;
            }
        }

        Ok(directives)
    }

    /// Consumes current token if current token is `,`.
    /// Returns `Err` if current token is neither `,` nor `list_end`.
    fn consume_list_delimiter(&mut self, list_end: &TokenValue) -> Result<()> {
        if !self.maybe_consume(&punct!(","))? && !self.current_matches(list_end) {
            let token = self.consume()?;
            return Err(Error::expected_other_token(token, &punct!(",")));
        }

        Ok(())
    }

    /// `true` if the current token is not preceded by a line feed or is a semicolon.
    fn stmt_ended(&self) -> bool {
        match self.current() {
            token_matches!(ok: punct!(";")) | Err(_) => true,
            _ => self.can_insert_semicolon(),
        }
    }

    /// Check if it is valid to insert semicolon before the current token.
    fn can_insert_semicolon(&self) -> bool {
        self.is_end()
            || self.current_matches(&punct!("}"))
            || self
                .reader
                .current()
                .map(|t| t.first_on_line)
                .unwrap_or(false)
    }
}

/// Returns `true` if provided `token` could be parsed to a valid identifier, and not a reserved
/// word in provided context.
fn is_identifier(token: Option<&Token>, keyword_context: KeywordContext) -> bool {
    match token {
        Some(Token {
            value: TokenValue::Identifier(_),
            ..
        }) => true,
        Some(Token {
            value: TokenValue::Keyword(keyword),
            ..
        }) => keyword.is_allowed_as_identifier(keyword_context),
        _ => false,
    }
}
