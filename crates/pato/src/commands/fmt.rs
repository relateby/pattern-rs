use crate::editor;
use crate::source_map;
use gram_codec::cst::{parse_gram_cst, ArrowKind, SyntaxKind, SyntaxNode};
use gram_codec::Pattern;
use pattern_core::{Subject, Value};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub struct FmtOutcome {
    pub exit_code: i32,
    pub stdout: Option<String>,
}

pub fn format_gram(source: &str) -> io::Result<String> {
    format_source_internal("<generated>", source, false)
        .map_err(|message| io::Error::new(io::ErrorKind::Other, message))
}

pub fn format_paths(files: &[PathBuf], check: bool) -> FmtOutcome {
    if files.iter().any(|file| file == Path::new("-")) {
        if files.len() != 1 {
            eprintln!("stdin mode cannot be combined with file paths");
            return FmtOutcome {
                exit_code: 3,
                stdout: None,
            };
        }

        if check {
            eprintln!("--check is not supported with stdin");
            return FmtOutcome {
                exit_code: 3,
                stdout: None,
            };
        }

        return match read_stdin() {
            Ok(source) => match format_source("<stdin>", &source) {
                Ok(formatted) => FmtOutcome {
                    exit_code: 0,
                    stdout: Some(formatted),
                },
                Err(()) => FmtOutcome {
                    exit_code: 2,
                    stdout: None,
                },
            },
            Err(error) => {
                eprintln!("failed to read stdin: {error}");
                FmtOutcome {
                    exit_code: 3,
                    stdout: None,
                }
            }
        };
    }

    let mut had_parse_error = false;
    let mut had_io_error = false;
    let mut needs_changes = false;

    for file in files {
        let source = match fs::read_to_string(file) {
            Ok(source) => source,
            Err(error) => {
                had_io_error = true;
                eprintln!("failed to read {}: {error}", file.display());
                continue;
            }
        };

        let formatted = match format_source(&file.display().to_string(), &source) {
            Ok(formatted) => formatted,
            Err(()) => {
                had_parse_error = true;
                continue;
            }
        };

        if formatted == source {
            continue;
        }

        if check {
            needs_changes = true;
            eprintln!("{}", file.display());
            continue;
        }

        match editor::write_atomic(file, &formatted) {
            Ok(()) => eprintln!("modified {}", file.display()),
            Err(error) => {
                had_io_error = true;
                eprintln!("failed to rewrite {}: {error}", file.display());
            }
        }
    }

    let exit_code = if had_io_error {
        3
    } else if had_parse_error {
        2
    } else if check && needs_changes {
        1
    } else {
        0
    };

    FmtOutcome {
        exit_code,
        stdout: None,
    }
}

fn format_source(file_name: &str, source: &str) -> Result<String, ()> {
    format_source_internal(file_name, source, true).map_err(|_| ())
}

fn format_source_internal(
    file_name: &str,
    source: &str,
    report_errors: bool,
) -> Result<String, String> {
    let parse_result = parse_gram_cst(source);
    if !parse_result.is_valid() {
        if report_errors {
            report_parse_errors(file_name, source, &parse_result.errors);
        }
        return Err(format!("failed to parse {file_name}"));
    }

    let mut items = Vec::new();
    if let Some(header) = parse_result.tree.value.subject.as_ref() {
        items.push(render_record(header.properties()));
    }

    for element in &parse_result.tree.elements {
        items.push(render_top_level_element(source, element));
    }

    let body = items
        .into_iter()
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n");

    if body.is_empty() {
        Ok(body)
    } else {
        Ok(format!("{body}\n"))
    }
}

fn report_parse_errors(file_name: &str, source: &str, errors: &[gram_codec::cst::SourceSpan]) {
    if errors.is_empty() {
        eprintln!("failed to parse {file_name}");
        return;
    }

    for span in errors {
        let location = source_map::span_start_location(source, span);
        eprintln!(
            "failed to parse {file_name} at {}:{}",
            location.line, location.column
        );
    }
}

fn render_top_level_element(source: &str, node: &Pattern<SyntaxNode>) -> String {
    match node.value.kind {
        SyntaxKind::Comment => source_map::span_text(source, &node.value.span)
            .trim()
            .to_string(),
        _ => render_pattern(source, node),
    }
}

fn render_pattern(source: &str, node: &Pattern<SyntaxNode>) -> String {
    match &node.value.kind {
        SyntaxKind::Document => String::new(),
        SyntaxKind::Comment => source_map::span_text(source, &node.value.span)
            .trim()
            .to_string(),
        SyntaxKind::Node => {
            format!(
                "({})",
                render_subject(
                    node.value.subject.as_ref(),
                    node_subject_text(source, node),
                    LabelMode::NodeLike,
                )
            )
        }
        SyntaxKind::Subject => render_subject_pattern(source, node),
        SyntaxKind::Relationship(arrow_kind) => render_relationship(source, node, arrow_kind),
        SyntaxKind::Annotated => render_annotated(source, node),
    }
}

fn render_subject_pattern(source: &str, node: &Pattern<SyntaxNode>) -> String {
    let subject = render_subject(
        node.value.subject.as_ref(),
        subject_pattern_subject_text(source, node),
        LabelMode::NodeLike,
    );
    let elements = node
        .elements
        .iter()
        .map(|element| render_pattern(source, element))
        .collect::<Vec<_>>()
        .join(", ");

    match (subject.is_empty(), elements.is_empty()) {
        (true, true) => "[]".to_string(),
        (false, true) => format!("[{subject}]"),
        (true, false) => format!("[| {elements}]"),
        (false, false) => format!("[{subject} | {elements}]"),
    }
}

fn render_relationship(source: &str, node: &Pattern<SyntaxNode>, arrow_kind: &ArrowKind) -> String {
    let left = node
        .elements
        .first()
        .map(|element| render_pattern(source, element))
        .unwrap_or_default();
    let right = node
        .elements
        .get(1)
        .map(|element| render_pattern(source, element))
        .unwrap_or_default();
    let arrow_text = relationship_span_text(source, node);
    let family = detect_arrow_family(arrow_text);
    let edge_subject = node.value.subject.as_ref().map(|subject| {
        render_subject(
            Some(subject),
            arrow_subject_text(arrow_text),
            LabelMode::Relationship,
        )
    });
    let arrow = build_arrow(arrow_kind, family, edge_subject.as_deref());
    format!("{left}{arrow}{right}")
}

fn render_annotated(source: &str, node: &Pattern<SyntaxNode>) -> String {
    let Some(inner) = node.elements.first() else {
        return String::new();
    };

    let prefix =
        &source[node.value.span.start.min(source.len())..inner.value.span.start.min(source.len())];
    let annotations = scan_annotation_tokens(prefix).join(" ");
    let inner_rendered = render_pattern(source, inner);
    if annotations.is_empty() {
        inner_rendered
    } else {
        format!("{annotations} {inner_rendered}")
    }
}

fn render_subject(
    subject: Option<&Subject>,
    raw_subject: Option<&str>,
    label_mode: LabelMode,
) -> String {
    let Some(subject) = subject else {
        return String::new();
    };

    let mut result = String::new();
    if !subject.identity.0.is_empty() {
        result.push_str(&quote_identifier(&subject.identity.0));
    }

    let raw_labels = raw_subject.map(scan_labels).unwrap_or_default();
    if !raw_labels.is_empty() {
        for label in raw_labels {
            result.push_str(&label.separator);
            result.push_str(&quote_identifier(&normalize_label(&label.name, label_mode)));
        }
    } else if !subject.labels.is_empty() {
        let separator = raw_subject
            .and_then(primary_label_separator)
            .unwrap_or_else(|| ":".to_string());
        let mut labels: Vec<_> = subject.labels.iter().cloned().collect();
        labels.sort();
        for label in labels {
            result.push_str(&separator);
            result.push_str(&quote_identifier(&normalize_label(&label, label_mode)));
        }
    }

    if !subject.properties.is_empty() {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&render_record(&subject.properties));
    }

    result
}

fn render_record(properties: &HashMap<String, Value>) -> String {
    if properties.is_empty() {
        return "{}".to_string();
    }

    let mut entries: Vec<_> = properties.iter().collect();
    entries.sort_by(|left, right| left.0.cmp(right.0));
    let body = entries
        .into_iter()
        .map(|(key, value)| format!("{}: {}", quote_identifier(key), render_value(value)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{{{body}}}")
}

fn render_value(value: &Value) -> String {
    match value {
        Value::VInteger(value) => value.to_string(),
        Value::VDecimal(value) => gram_codec::Value::Decimal(*value).to_gram_notation(),
        Value::VBoolean(value) => value.to_string(),
        Value::VString(value) => gram_codec::Value::String(value.clone()).to_gram_notation(),
        Value::VSymbol(value) => gram_codec::Value::String(value.clone()).to_gram_notation(),
        Value::VTaggedString { tag, content } => gram_codec::Value::TaggedString {
            tag: tag.clone(),
            content: content.clone(),
        }
        .to_gram_notation(),
        Value::VArray(values) => {
            let items = values
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{items}]")
        }
        Value::VMap(entries) => {
            let mut entries: Vec<_> = entries.iter().collect();
            entries.sort_by(|left, right| left.0.cmp(right.0));
            let body = entries
                .into_iter()
                .map(|(key, value)| format!("{}: {}", quote_identifier(key), render_value(value)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{body}}}")
        }
        Value::VRange(range) => {
            let lower = range.lower.unwrap_or_default();
            let upper = range.upper.unwrap_or_default();
            if lower.fract() == 0.0 && upper.fract() == 0.0 {
                format!("{}..{}", lower as i64, upper as i64)
            } else {
                format!("{lower}..{upper}")
            }
        }
        Value::VMeasurement { unit, value } => format!("{value}{unit}"),
    }
}

fn node_subject_text<'a>(source: &'a str, node: &Pattern<SyntaxNode>) -> Option<&'a str> {
    let fragment = source_map::span_text(source, &node.value.span);
    if fragment.len() < 2 {
        return None;
    }
    Some(fragment[1..fragment.len() - 1].trim())
}

fn subject_pattern_subject_text<'a>(
    source: &'a str,
    node: &Pattern<SyntaxNode>,
) -> Option<&'a str> {
    let fragment = source_map::span_text(source, &node.value.span);
    if fragment.len() < 2 {
        return None;
    }

    let inner = &fragment[1..fragment.len() - 1];
    let split = inner.find('|').unwrap_or(inner.len());
    Some(inner[..split].trim())
}

fn relationship_span_text<'a>(source: &'a str, node: &'a Pattern<SyntaxNode>) -> &'a str {
    let Some(left) = node.elements.first() else {
        return "";
    };
    let Some(right) = node.elements.get(1) else {
        return "";
    };
    let start = left.value.span.end.min(source.len());
    let end = right.value.span.start.min(source.len());
    if start > end {
        ""
    } else {
        &source[start..end]
    }
}

fn arrow_subject_text(arrow_text: &str) -> Option<&str> {
    let start = arrow_text.find('[')?;
    let end = arrow_text.rfind(']')?;
    (start < end).then_some(arrow_text[start + 1..end].trim())
}

fn detect_arrow_family(arrow_text: &str) -> char {
    if arrow_text.contains('=') {
        '='
    } else if arrow_text.contains('~') {
        '~'
    } else {
        '-'
    }
}

fn build_arrow(kind: &ArrowKind, family: char, subject: Option<&str>) -> String {
    let bracketed = subject
        .filter(|subject| !subject.is_empty())
        .map(|subject| format!("[{subject}]"))
        .unwrap_or_default();

    match (kind, family) {
        (ArrowKind::Right, '-') => format!("-{bracketed}->"),
        (ArrowKind::Right, '=') => format!("={bracketed}=>"),
        (ArrowKind::Right, '~') => format!("~{bracketed}~>"),
        (ArrowKind::Left, '-') => format!("<-{bracketed}-"),
        (ArrowKind::Left, '=') => format!("<={bracketed}="),
        (ArrowKind::Left, '~') => format!("<~{bracketed}~"),
        (ArrowKind::Bidirectional, '-') => format!("<-{bracketed}->"),
        (ArrowKind::Bidirectional, '=') => format!("<={bracketed}=>"),
        (ArrowKind::Bidirectional, '~') => format!("<~{bracketed}~>"),
        (ArrowKind::Undirected, '-') => format!("-{bracketed}-"),
        (ArrowKind::Undirected, '=') => format!("={bracketed}="),
        (ArrowKind::Undirected, '~') => format!("~{bracketed}~"),
        _ => format!("-{bracketed}->"),
    }
}

#[derive(Clone, Copy)]
enum LabelMode {
    NodeLike,
    Relationship,
}

#[derive(Clone)]
struct RawLabel {
    separator: String,
    name: String,
}

fn scan_labels(raw_subject: &str) -> Vec<RawLabel> {
    let mut labels = Vec::new();
    let prefix = raw_subject
        .split_once('{')
        .map(|(before, _)| before)
        .unwrap_or(raw_subject);
    let bytes = prefix.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'"' => index = skip_quoted(prefix, index + 1),
            b':' => {
                let separator = if bytes.get(index + 1) == Some(&b':') {
                    index += 2;
                    "::"
                } else {
                    index += 1;
                    ":"
                };
                index = skip_ascii_whitespace(prefix, index);
                let (name, next) = scan_identifier(prefix, index);
                if !name.is_empty() {
                    labels.push(RawLabel {
                        separator: separator.to_string(),
                        name,
                    });
                }
                index = next;
            }
            _ => index += 1,
        }
    }

    labels
}

fn primary_label_separator(raw_subject: &str) -> Option<String> {
    scan_labels(raw_subject)
        .first()
        .map(|label| label.separator.clone())
}

fn scan_annotation_tokens(prefix: &str) -> Vec<String> {
    let bytes = prefix.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] != b'@' {
            index += 1;
            continue;
        }

        let start = index;
        index += 1;
        if bytes.get(index) == Some(&b'@') {
            index += 1;
            while index < bytes.len() {
                match bytes[index] {
                    b'"' => index = skip_quoted(prefix, index + 1),
                    b'@' => break,
                    _ => index += 1,
                }
            }
        } else {
            let mut depth = 0u32;
            while index < bytes.len() {
                match bytes[index] {
                    b'"' => index = skip_quoted(prefix, index + 1),
                    b'(' => {
                        depth += 1;
                        index += 1;
                    }
                    b')' => {
                        depth = depth.saturating_sub(1);
                        index += 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    b'@' if depth == 0 => break,
                    _ => index += 1,
                }
            }
        }

        let token = prefix[start..index].trim();
        if !token.is_empty() {
            tokens.push(token.to_string());
        }
    }

    tokens
}

fn scan_identifier(source: &str, start: usize) -> (String, usize) {
    let bytes = source.as_bytes();
    if start >= bytes.len() {
        return (String::new(), start);
    }

    if bytes[start] == b'"' {
        let end = skip_quoted(source, start + 1).min(bytes.len());
        return (source[start..end].trim().to_string(), end);
    }

    let mut end = start;
    while end < bytes.len() {
        let ch = bytes[end] as char;
        if ch.is_ascii_whitespace() || matches!(ch, ':' | '{' | '}' | '[' | ']' | '(' | ')' | ',') {
            break;
        }
        end += 1;
    }
    (source[start..end].trim().to_string(), end)
}

fn skip_quoted(source: &str, mut index: usize) -> usize {
    let bytes = source.as_bytes();
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index = index.saturating_add(2),
            b'"' => return index + 1,
            _ => index += 1,
        }
    }
    bytes.len()
}

fn skip_ascii_whitespace(source: &str, mut index: usize) -> usize {
    let bytes = source.as_bytes();
    while index < bytes.len() && (bytes[index] as char).is_ascii_whitespace() {
        index += 1;
    }
    index
}

fn normalize_label(label: &str, label_mode: LabelMode) -> String {
    match label_mode {
        LabelMode::NodeLike => title_case(label),
        LabelMode::Relationship => label.to_uppercase(),
    }
}

fn title_case(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => {
            let mut title = String::new();
            title.push(first.to_ascii_uppercase());
            title.push_str(&chars.as_str().to_ascii_lowercase());
            title
        }
        None => String::new(),
    }
}

fn quote_identifier(s: &str) -> String {
    if needs_quoting(s) {
        format!("\"{}\"", escape_string(s))
    } else {
        s.to_string()
    }
}

fn needs_quoting(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return true;
    }

    s.chars().any(|c| {
        !c.is_ascii()
            || c.is_whitespace()
            || matches!(
                c,
                '{' | '}' | '[' | ']' | '(' | ')' | ':' | ',' | '@' | '#' | '-' | '~' | '"'
            )
    })
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn read_stdin() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(input)
}

trait SubjectExt {
    fn properties(&self) -> &HashMap<String, Value>;
}

impl SubjectExt for Subject {
    fn properties(&self) -> &HashMap<String, Value> {
        &self.properties
    }
}
