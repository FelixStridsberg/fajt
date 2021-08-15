use crate::ast::{DeclImport, Import, Stmt};
use crate::error::Result;
use crate::Parser;
use fajt_common::io::PeekRead;
use fajt_lexer::keyword;
use fajt_lexer::token::Token;

impl<I> Parser<'_, I>
where
    I: PeekRead<Token, Error = fajt_lexer::error::Error>,
{
    // Parses the `ImportDeclaration` goal symbol.
    pub(super) fn parse_import_declaration(&mut self) -> Result<Stmt> {
        let span_start = self.position();
        self.consume_assert(keyword!("import"))?;

        let import = if self.current_matches_string_literal() {
            self.parse_import_module()?
        } else {
            todo!("Not yet implemented kind of module import")
        };

        let span = self.span_from(span_start);
        Ok(DeclImport { span, import }.into())
    }

    // Parses `import "module";` statement, after the import.
    fn parse_import_module(&mut self) -> Result<Import> {
        let (module_name, _) = self
            .parse_literal()?
            .unwrap_literal()
            .literal
            .unwrap_string();
        Ok(Import::Module(module_name))
    }
}
