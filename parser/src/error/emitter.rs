use crate::error::{expected_token_to_string, ErrorKind};
use crate::Error;
use fajt_ast::Span;
use std::io::Write;

pub struct ErrorEmitter<'a, 'b, 'c, W> {
    filename: &'a str,
    source: &'b str,
    out: &'c mut W,
}

impl<'a, 'b, 'c, W: Write> ErrorEmitter<'a, 'b, 'c, W> {
    pub fn new(filename: &'a str, source: &'b str, out: &'c mut W) -> ErrorEmitter<'a, 'b, 'c, W> {
        ErrorEmitter {
            filename,
            source,
            out,
        }
    }

    pub fn emit_error(&mut self, error: &Error) -> std::io::Result<()> {
        let span = &error.span;
        let line_span = self.get_line_boundaries(span);
        let col_number = span.start - line_span.start + 1;
        let line_number = self.get_line_number(span);

        writeln!(self.out, "{}", error)?;

        if span.is_empty() {
            writeln!(self.out, " --> {}", self.filename)?;
        } else {
            writeln!(
                self.out,
                " --> {}:{}:{}",
                self.filename, line_number, col_number
            )?;
        }

        if error.kind != ErrorKind::EndOfStream {
            let label = self.get_kind_description(error);
            self.emit_diagnostic(error, &label, line_number, line_span)?;
        }

        Ok(())
    }

    pub fn emit_diagnostic(
        &mut self,
        error: &Error,
        label: &str,
        line_number: usize,
        line_span: Span,
    ) -> std::io::Result<()> {
        let line_number_str = line_number.to_string();
        let padding = line_number_str.len() + 1;

        let err_span = error.span.translate(-(line_span.start as isize));
        let err_token_length = err_span.end - err_span.start;

        writeln!(self.out, "{:<pad$}|", " ", pad = padding)?;

        let line = &self.source[line_span.start..line_span.end];
        let tab_indents = &line[0..err_span.start].matches('\t').count();
        writeln!(
            self.out,
            "{:<pad$}| {}",
            line_number_str,
            line.replace('\t', "    "),
            pad = padding
        )?;
        writeln!(
            self.out,
            "{:<pad$}| {:<err_offset$}{:^<err_mark$} {}",
            "",
            "",
            "^",
            label,
            pad = padding,
            err_offset = err_span.start + tab_indents * 3,
            err_mark = err_token_length
        )?;

        Ok(())
    }

    fn get_line_number(&self, span: &Span) -> usize {
        self.source[..span.end].matches('\n').count() + 1
    }

    fn get_line_boundaries(&self, span: &Span) -> Span {
        let start = self.source[..span.start]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        let end = self.source[span.end..]
            .find('\n')
            .map(|pos| pos + span.end)
            .unwrap_or(self.source.len());

        Span::new(start, end)
    }

    fn get_kind_description(&self, error: &Error) -> String {
        match &error.kind {
            ErrorKind::ForbiddenIdentifier(keyword) => {
                format!("`{keyword}` is not allowed as an identifier in this context")
            }
            ErrorKind::UnexpectedToken(_, None) => "Unexpected token".to_string(),
            ErrorKind::UnexpectedToken(_, Some(expected)) => {
                let token_value = &self.source[error.span.start..error.span.end];
                format!(
                    "Unexpected token, found `{token_value}`, expected `{}`",
                    expected_token_to_string(expected).unwrap_or_default(),
                )
            }
            ErrorKind::ExpectedIdentifier(expected) => format!(
                "Unexpected token, found `{}`, expected identifier",
                expected_token_to_string(expected).unwrap_or_default(),
            ),
            _ => String::new(),
        }
    }
}
