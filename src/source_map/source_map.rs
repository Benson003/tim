use crate::source_map::span::Span;

#[derive(Clone)]
pub struct SourceMap {
    pub newlines: Vec<usize>,
    pub source: String,
}

impl SourceMap {
    pub fn new(source: String) -> Self {
        let mut newlines = vec![0];

        for (offset, char) in source.char_indices() {
            if char == '\n' {
                newlines.push(offset + 1);
            }
        }

        Self { newlines, source }
    }

    pub fn lookup_char_postion(&self, pos: usize) -> (usize, usize) {
        let line_idx = self.newlines.partition_point(|&offset| offset <= pos) - 1;
        let line_start = self.newlines[line_idx];

        let col_idx = pos - line_start;

        (line_idx, col_idx)
    }

    pub fn snippet(&self, span: Span) -> &str {
        &self.source[span.lo..span.hi]
    }
}
