use pest::error::LineColLocation;
use pest::Span;

/// Maps byte offsets in the parser's input string to (line, column) pairs in O(log L)
/// instead of pest's default O(input length) per-call walk from the start of input.
pub struct LineIndex<'i> {
    input: &'i str,
    line_starts: Vec<usize>,
}

impl<'i> LineIndex<'i> {
    pub fn new(input: &'i str) -> Self {
        let mut line_starts = vec![0];
        for (i, b) in input.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Self { input, line_starts }
    }

    /// 1-based (line, col) for a byte offset, matching pest's `Position::line_col` semantics
    /// (column counts characters, not bytes).
    fn line_col(&self, byte_offset: usize) -> (usize, usize) {
        debug_assert!(byte_offset <= self.input.len());
        let line_idx = self
            .line_starts
            .binary_search(&byte_offset)
            .unwrap_or_else(|i| i - 1);
        let line_start = self.line_starts[line_idx];
        let col = self.input[line_start..byte_offset].chars().count() + 1;
        (line_idx + 1, col)
    }

    /// Build a `LineColLocation` for a span. Mirrors pest's `From<Span> for LineColLocation`
    /// (returns `Pos` when start==end, `Span` otherwise).
    pub fn linecol(&self, span: Span<'_>) -> LineColLocation {
        let start = self.line_col(span.start());
        let end = self.line_col(span.end());
        if start == end {
            LineColLocation::Pos(start)
        } else {
            LineColLocation::Span(start, end)
        }
    }
}
