use crate::ast::{DeclImport, Ident, NamedImport, Stmt};
use crate::error::{Result, ThenTry};
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::punct;
use fajt_lexer::token::Token;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    // Parses the `ImportDeclaration` goal symbol.
    pub(super) fn parse_import_declaration(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("import"))?;

        let (default_binding, named_imports, namespace_binding, source) = if self
            .current_matches_string_literal()
        {
            (None, None, None, self.parse_module_specifier()?)
        } else {
            let (default_binding, named_imports, namespace_binding) = self.parse_import_clause()?;
            self.consume_assert(keyword!("from"))?;
            let source = self.parse_module_specifier()?;
            (default_binding, named_imports, namespace_binding, source)
        };

        let span = self.span_from(span_start);
        Ok(DeclImport {
            span,
            default_binding,
            namespace_binding,
            named_imports,
            source,
        }
        .into())
    }

    /// Parses the `ModuleSpecifier` goal symbol.
    fn parse_module_specifier(&mut self) -> Result<String> {
        let (module_name, _) = self
            .parse_literal()?
            .unwrap_literal()
            .literal
            .unwrap_string();
        Ok(module_name)
    }

    fn parse_import_clause(
        &mut self,
    ) -> Result<(Option<Ident>, Option<Vec<NamedImport>>, Option<Ident>)> {
        let default_binding = self.is_identifier().then_try(|| self.parse_identifier())?;
        let namespace_binding = self
            .current_matches(punct!("*"))
            .then_try(|| self.parse_namespace_import())?;
        let named_imports = self
            .current_matches(punct!("{"))
            .then_try(|| self.parse_named_imports())?
            .map(Option::Some)
            .unwrap_or(None);

        Ok((default_binding, named_imports, namespace_binding))
    }

    /// Parses the `NameSpaceImport` goal symbol.
    fn parse_namespace_import(&mut self) -> Result<Ident> {
        self.consume_assert(punct!("*"))?;
        self.consume_assert(keyword!("as"))?;
        self.parse_identifier()
    }

    fn parse_named_imports(&mut self) -> Result<Vec<NamedImport>> {
        self.consume_assert(punct!("{"))?;

        let mut named_imports = Vec::new();
        loop {
            if self.current_matches(punct!("}")) {
                self.consume()?;
                break;
            }

            named_imports.push(self.parse_named_import()?);
            self.consume_object_delimiter()?;
        }

        Ok(named_imports)
    }

    fn parse_named_import(&mut self) -> Result<NamedImport> {
        let span_start = self.position();
        let name = self.parse_identifier()?;
        let alias = self
            .maybe_consume(keyword!("as"))?
            .then_try(|| self.parse_identifier())?;
        let span = self.span_from(span_start);
        Ok(NamedImport { span, name, alias })
    }
}
