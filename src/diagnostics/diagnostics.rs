use crate::source_map::{source_map::SourceMap, span::Span};

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
    Help,
}

impl Severity {
    fn to_str(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Help => "help",
        }
    }

    fn color_code(&self) -> &'static str {
        match self {
            Severity::Error => "\x1b[31;1m",   // Bold Red
            Severity::Warning => "\x1b[33;1m", // Bold Yellow
            Severity::Help => "\x1b[36;1m",    // Bold Cyan
        }
    }

    fn reset_code(&self) -> &'static str {
        "\x1b[0m"
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub label: Option<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            span,
            label: None,
        }
    }

    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            span,
            label: None,
        }
    }

    pub fn help(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Help,
            message: message.into(),
            span,
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

pub struct DiagnosticPrinter<'a> {
    source_map: &'a SourceMap,
}

impl<'a> DiagnosticPrinter<'a> {
    pub fn new(source_map: &'a SourceMap) -> Self {
        Self { source_map }
    }

    pub fn print(&self, diagnostic: &Diagnostic) {
        let (line_idx, col_idx) = self.source_map.lookup_char_postion(diagnostic.span.lo);
        let line_num = line_idx + 1;
        let col_num = col_idx + 1;

        // 1. Print the header
        println!(
            "{}{}{}: {}",
            diagnostic.severity.color_code(),
            diagnostic.severity.to_str(),
            diagnostic.severity.reset_code(),
            diagnostic.message
        );

        // 2. Print location info
        println!("  --> input:{}:{}", line_num, col_num);

        // 3. Get the line
        let line_start = self.source_map.newlines[line_idx];
        let line_end = self
            .source_map
            .newlines
            .get(line_idx + 1)
            .map(|&offset| {
                if offset > 0 && self.source_map.source.as_bytes()[offset - 1] == b'\n' {
                    offset - 1
                } else {
                    offset
                }
            })
            .unwrap_or(self.source_map.source.len());

        let line_text = &self.source_map.source[line_start..line_end];

        // 4. Print the source context
        let margin = format!(" {} |", line_num);
        let empty_margin = " ".repeat(margin.len() - 1) + "|";

        println!("{}", empty_margin);
        println!("{} {}", margin, line_text);

        // 5. Build the highlight
        let span_len = (diagnostic.span.hi - diagnostic.span.lo).max(1);
        let spaces = " ".repeat(col_idx);
        let carets = "^".repeat(span_len);

        if let Some(label) = &diagnostic.label {
            println!("{} {}{} {}", empty_margin, spaces, carets, label);
        } else {
            println!("{} {}{}", empty_margin, spaces, carets);
        }
        println!("{}", empty_margin);
        println!();
    }

    pub fn print_multiple(&self, diagnostics: &[&Diagnostic]) {
        for diagnostic in diagnostics {
            self.print(diagnostic);
        }
    }
}
