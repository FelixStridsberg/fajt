use fajt_ast::Span;

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
}

pub struct DiagnosticEmitter<'a> {
    output: String, // TODO
    source: &'a str,
}

impl<'a> DiagnosticEmitter<'a> {
    pub fn new(source: &'a str) -> Self {
        DiagnosticEmitter {
            output: String::new(),
            source,
        }
    }

    pub fn into_string(self) -> String {
        self.output
    }

    pub fn emit_diagnostic(&mut self, diagnostic: &Diagnostic) {
        let line_number = self.get_row_number(&diagnostic.span);
        let (line, span) = self.get_line(&diagnostic.span);

        let line_number_str = line_number.to_string();

        let mut output = String::new();
        let padding = line_number_str.len() + 1;
        output.push_str(&" ".repeat(padding));
        output.push_str("|\n");

        output.push_str(&format!("{:<width$}| ", width = padding));
        output.push_str(&format!("{}\n", line));

        output.push_str(&" ".repeat(padding));
        output.push_str("| ");
        output.push_str(&" ".repeat(span.start));
        output.push_str(&"^".repeat(span.end - span.start));
        output.push(' ');
        output.push_str(&diagnostic.message);

        self.output.push_str(&output);
    }

    fn get_line(&self, span: &Span) -> (&str, Span) {
        let start = self.source[..span.start]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        let end = self.source[span.end..]
            .find('\n')
            .map(|pos| pos + span.end)
            .unwrap_or(self.source.len());
        let line = &self.source[start..end];
        let line_span = Span::new(span.start - start, span.end - start);
        (line, line_span)
    }

    fn get_row_number(&self, span: &Span) -> usize {
        self.source[..span.end].matches('\n').count()
    }
}
