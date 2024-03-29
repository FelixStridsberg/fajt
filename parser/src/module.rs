use crate::error::Result;
use crate::{Error, Parser, ThenTry};
use fajt_ast::{
    DeclExport, DeclImport, ExportDecl, ExportDefaultDecl, ExportDefaultExpr, ExportNamed,
    ExportNamespace, Ident, LitString, NamedExport, NamedImport, Stmt, VariableKind,
};
use fajt_common::io::{PeekRead, ReReadWithState};
use fajt_lexer::punct;
use fajt_lexer::token::Token;
use fajt_lexer::token_matches;
use fajt_lexer::{keyword, LexerState};

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
    I: ReReadWithState<Token, State = LexerState, Error = fajt_lexer::error::Error>,
{
    /// Parses the `ExportDeclaration` production.
    pub(super) fn parse_export_declaration(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("export"))?;

        match self.current() {
            token_matches!(ok: punct!("{")) => self.parse_named_export(span_start),
            token_matches!(ok: punct!("*")) => self.parse_namespace_export(span_start),
            token_matches!(ok: keyword!("var")) => self.parse_varible_statement_export(span_start),
            token_matches!(ok: keyword!("let"))
            | token_matches!(ok: keyword!("const"))
            | token_matches!(ok: keyword!("function"))
            | token_matches!(ok: keyword!("class")) => self.parse_declaration_export(span_start),
            token_matches!(ok: keyword!("async")) if self.peek_matches(&keyword!("function")) => {
                self.parse_declaration_export(span_start)
            }
            token_matches!(ok: keyword!("default")) => self.parse_default_export(span_start),
            _ => Err(Error::unexpected_token(self.consume()?)),
        }
    }

    fn parse_default_export(&mut self, span_start: usize) -> Result<Stmt> {
        self.consume_assert(&keyword!("default"))?;
        match self.current()? {
            token_matches!(keyword!("class")) | token_matches!(keyword!("function")) => self
                .with_context(self.context.with_default(true))
                .parse_declaration_default_export(span_start),
            token_matches!(keyword!("async")) if self.peek_matches(&keyword!("function")) => self
                .with_context(self.context.with_default(true))
                .parse_declaration_default_export(span_start),
            _ => {
                let expr = self
                    .with_context(self.context.with_in(true))
                    .parse_assignment_expr()?;
                self.consume_optional_semicolon()?;
                let span = self.span_from(span_start);
                Ok(DeclExport::DefaultExpr(ExportDefaultExpr {
                    span,
                    expr: Box::new(expr),
                })
                .into())
            }
        }
    }

    /// Parses any `export` followed by a `var`.
    fn parse_varible_statement_export(&mut self, span_start: usize) -> Result<Stmt> {
        let stmt = self.parse_variable_stmt(VariableKind::Var)?;
        let span = self.span_from(span_start);
        Ok(DeclExport::Decl(ExportDecl {
            span,
            decl: Box::new(stmt),
        })
        .into())
    }

    /// Parses any `export` followed by a declaration.
    fn parse_declaration_export(&mut self, span_start: usize) -> Result<Stmt> {
        let decl = self.parse_required_declaration()?;
        let span = self.span_from(span_start);
        Ok(DeclExport::Decl(ExportDecl {
            span,
            decl: Box::new(decl),
        })
        .into())
    }

    /// Parses any `export default` followed by a declaration.
    fn parse_declaration_default_export(&mut self, span_start: usize) -> Result<Stmt> {
        let decl = self.parse_required_declaration()?;
        let span = self.span_from(span_start);
        Ok(DeclExport::DefaultDecl(ExportDefaultDecl {
            span,
            decl: Box::new(decl),
        })
        .into())
    }

    fn parse_required_declaration(&mut self) -> Result<Stmt> {
        let decl = self.parse_declaration()?;
        if let Some(decl) = decl {
            Ok(decl)
        } else {
            let span = self.span_from(self.position());
            Err(Error::syntax_error("Expected statement".to_owned(), span))
        }
    }

    /// Parses `export * from 'module'` and `export * as alias from 'module'`.
    fn parse_namespace_export(&mut self, span_start: usize) -> Result<Stmt> {
        self.consume_assert(&punct!("*"))?;
        let alias = self
            .maybe_consume(&keyword!("as"))?
            .then_try(|| self.parse_identifier_name())?;
        self.consume_assert(&keyword!("from"))?;
        let from = self.parse_module_specifier()?;
        self.consume_optional_semicolon()?;
        let span = self.span_from(span_start);
        Ok(DeclExport::Namespace(ExportNamespace { span, alias, from }).into())
    }

    /// Parses `export { name }` and `export { name as name2 } from 'other'`.
    fn parse_named_export(&mut self, span_start: usize) -> Result<Stmt> {
        let named_exports = self.parse_named_exports()?;
        let from = self
            .maybe_consume(&keyword!("from"))?
            .then_try(|| self.parse_module_specifier())?;
        self.consume_optional_semicolon()?;
        let span = self.span_from(span_start);
        Ok(DeclExport::Named(ExportNamed {
            span,
            named_exports,
            from,
        })
        .into())
    }

    /// Parses the `ImportDeclaration` production.
    pub(super) fn parse_import_declaration(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(&keyword!("import"))?;

        // `import "./module.js"`;
        if self.current_matches_string_literal() {
            return self.parse_module_import(span_start);
        }

        let import_clause = self.parse_import_clause()?;

        self.consume_assert(&keyword!("from"))?;
        let from = self.parse_module_specifier()?;

        self.consume_optional_semicolon()?;

        let span = self.span_from(span_start);
        Ok(DeclImport {
            span,
            default_binding: import_clause.default_binding,
            namespace_binding: import_clause.namespace_binding,
            named_imports: import_clause.named_imports,
            from,
        }
        .into())
    }

    /// Parses the `ImportClause` production.
    fn parse_import_clause(&mut self) -> Result<ImportClause> {
        let default_binding = self.is_identifier().then_try(|| self.parse_identifier())?;
        let mut clause = ImportClause {
            default_binding,
            namespace_binding: None,
            named_imports: None,
        };

        if clause.default_binding.is_none() || self.maybe_consume(&punct!(","))? {
            match self.current() {
                token_matches!(ok: punct!("*")) => {
                    clause.namespace_binding = Some(self.parse_namespace_import()?)
                }
                token_matches!(ok: punct!("{")) => {
                    clause.named_imports = Some(self.parse_named_imports()?)
                }
                _ => return Err(Error::unexpected_token(self.consume()?)),
            }
        }

        Ok(clause)
    }

    fn parse_module_import(&mut self, span_start: usize) -> Result<Stmt> {
        let from = self.parse_module_specifier()?;
        self.consume_optional_semicolon()?;

        let span = self.span_from(span_start);
        Ok(DeclImport {
            span,
            default_binding: None,
            namespace_binding: None,
            named_imports: None,
            from,
        }
        .into())
    }

    /// Parses the `ModuleSpecifier` production.
    fn parse_module_specifier(&mut self) -> Result<LitString> {
        let module_name = self
            .parse_literal()?
            .unwrap_literal()
            .literal
            .unwrap_string();
        Ok(module_name)
    }

    /// Parses the `NameSpaceImport` production.
    fn parse_namespace_import(&mut self) -> Result<Ident> {
        self.consume_assert(&punct!("*"))?;
        self.consume_assert(&keyword!("as"))?;
        self.parse_identifier()
    }

    /// Parses the `NamedImports` production.
    fn parse_named_imports(&mut self) -> Result<Vec<NamedImport>> {
        self.consume_assert(&punct!("{"))?;

        let mut named_imports = Vec::new();
        loop {
            if self.current_matches(&punct!("}")) {
                self.consume()?;
                break;
            }

            named_imports.push(self.parse_import_specifier()?);
            self.consume_list_delimiter(&punct!("}"))?;
        }

        Ok(named_imports)
    }

    /// Parses the `ImportSpecifier` production.
    fn parse_import_specifier(&mut self) -> Result<NamedImport> {
        let span_start = self.position();
        let name = self.parse_identifier_name()?;
        let alias = self
            .maybe_consume(&keyword!("as"))?
            .then_try(|| self.parse_identifier())?;
        let span = self.span_from(span_start);
        Ok(NamedImport { span, name, alias })
    }

    /// Parses the `NamedExports` production.
    fn parse_named_exports(&mut self) -> Result<Vec<NamedExport>> {
        self.consume_assert(&punct!("{"))?;

        let mut named_exports = Vec::new();
        loop {
            if self.current_matches(&punct!("}")) {
                self.consume()?;
                break;
            }

            named_exports.push(self.parse_export_specifier()?);
            self.consume_list_delimiter(&punct!("}"))?;
        }

        Ok(named_exports)
    }
    /// Parses the `ExportSpecifier` production.
    fn parse_export_specifier(&mut self) -> Result<NamedExport> {
        let span_start = self.position();
        let mut name = self.parse_identifier_name()?;

        // If there is an alias, we swap the name and alias identifiers, since the name should be
        // the name of the export, and the alias the local name.
        let alias_of = self
            .maybe_consume(&keyword!("as"))?
            .then_try(|| self.parse_identifier_name())?
            .map(|alias| std::mem::replace(&mut name, alias));

        let span = self.span_from(span_start);
        Ok(NamedExport {
            span,
            name,
            alias_of,
        })
    }
}

struct ImportClause {
    pub default_binding: Option<Ident>,
    pub namespace_binding: Option<Ident>,
    pub named_imports: Option<Vec<NamedImport>>,
}
