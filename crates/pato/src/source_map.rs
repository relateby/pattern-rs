use crate::diagnostics::Location;
use gram_codec::cst::SourceSpan;
use std::cmp::Ordering;

pub fn offset_to_location(source: &str, offset: usize) -> Location {
    let offset = offset.min(source.len());
    let prefix = &source[..offset];

    let line = prefix.matches('\n').count() as u32 + 1;
    let column = prefix
        .rfind('\n')
        .map(|position| (offset - position) as u32)
        .unwrap_or(offset as u32 + 1);

    Location::new(line, column)
}

pub fn location_at_span_offset(
    source: &str,
    span: &SourceSpan,
    relative_offset: usize,
) -> Location {
    offset_to_location(source, span.start.saturating_add(relative_offset))
}

pub fn span_start_location(source: &str, span: &SourceSpan) -> Location {
    offset_to_location(source, span.start)
}

pub fn span_text<'a>(source: &'a str, span: &SourceSpan) -> &'a str {
    let start = span.start.min(source.len());
    let end = span.end.min(source.len());
    if start > end {
        return &source[0..0];
    }
    &source[start..end]
}

pub fn compare_spans(left: &SourceSpan, right: &SourceSpan) -> Ordering {
    (left.start, left.end).cmp(&(right.start, right.end))
}
