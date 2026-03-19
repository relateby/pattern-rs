use crate::cli::ParseOutputFormatArg;
use crate::commands::fmt;
use crate::source_map;
use gram_codec::ast::AstPattern;
use gram_codec::cst::{parse_gram_cst, Annotation, ArrowKind, SourceSpan, SyntaxKind, SyntaxNode};
use gram_codec::{lower, to_gram, Pattern, Subject, Value as GramValue};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseOutputFormat {
    Gram,
    Sexp,
    Json,
    Summary,
}

impl From<ParseOutputFormatArg> for ParseOutputFormat {
    fn from(value: ParseOutputFormatArg) -> Self {
        match value {
            ParseOutputFormatArg::Gram => ParseOutputFormat::Gram,
            ParseOutputFormatArg::Sexp => ParseOutputFormat::Sexp,
            ParseOutputFormatArg::Json => ParseOutputFormat::Json,
            ParseOutputFormatArg::Summary => ParseOutputFormat::Summary,
        }
    }
}

pub struct ParseOutcome {
    pub exit_code: i32,
    pub stdout: Option<String>,
}

struct ParsedInput {
    file_name: String,
    source: String,
    tree: Pattern<SyntaxNode>,
    lowered: Vec<Pattern<Subject>>,
}

pub fn parse_paths(files: &[PathBuf], output_format: ParseOutputFormat) -> ParseOutcome {
    if files.iter().any(|file| file == Path::new("-")) {
        if files.len() != 1 {
            eprintln!("stdin mode cannot be combined with file paths");
            return ParseOutcome {
                exit_code: 3,
                stdout: None,
            };
        }

        return match read_stdin() {
            Ok(source) => match parse_source("<stdin>", source) {
                Ok(parsed) => ParseOutcome {
                    exit_code: 0,
                    stdout: Some(render_outputs(&[parsed], output_format)),
                },
                Err(()) => ParseOutcome {
                    exit_code: 2,
                    stdout: None,
                },
            },
            Err(error) => {
                eprintln!("failed to read stdin: {error}");
                ParseOutcome {
                    exit_code: 3,
                    stdout: None,
                }
            }
        };
    }

    let mut parsed = Vec::new();
    let mut had_parse_error = false;
    let mut had_io_error = false;

    for file in files {
        match fs::read_to_string(file) {
            Ok(source) => match parse_source(&file.display().to_string(), source) {
                Ok(result) => parsed.push(result),
                Err(()) => had_parse_error = true,
            },
            Err(error) => {
                had_io_error = true;
                eprintln!("failed to read {}: {error}", file.display());
            }
        }
    }

    let exit_code = if had_io_error {
        3
    } else if had_parse_error {
        2
    } else {
        0
    };

    let stdout = (!parsed.is_empty()).then(|| render_outputs(&parsed, output_format));

    ParseOutcome { exit_code, stdout }
}

fn parse_source(file_name: &str, source: String) -> Result<ParsedInput, ()> {
    let parse_result = parse_gram_cst(&source);
    if !parse_result.is_valid() {
        report_parse_errors(file_name, &source, &parse_result.errors);
        return Err(());
    }

    let tree = parse_result.tree;
    let lowered = lower(tree.clone());

    Ok(ParsedInput {
        file_name: file_name.to_string(),
        source,
        tree,
        lowered,
    })
}

fn report_parse_errors(file_name: &str, source: &str, errors: &[SourceSpan]) {
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

fn render_outputs(parsed: &[ParsedInput], output_format: ParseOutputFormat) -> String {
    match output_format {
        ParseOutputFormat::Gram => render_gram(parsed),
        ParseOutputFormat::Sexp => render_sexp(parsed),
        ParseOutputFormat::Json => render_json(parsed),
        ParseOutputFormat::Summary => render_summary(parsed),
    }
}

fn render_gram(parsed: &[ParsedInput]) -> String {
    let patterns = parsed
        .iter()
        .flat_map(|input| input.lowered.iter().cloned())
        .collect::<Vec<_>>();

    if patterns.is_empty() {
        return String::new();
    }

    let raw = to_gram(&patterns).expect("lowered patterns should serialize to gram");
    fmt::format_gram(&raw).expect("generated gram output should be canonical")
}

fn render_json(parsed: &[ParsedInput]) -> String {
    let ast = parsed
        .iter()
        .flat_map(|input| input.lowered.iter().map(AstPattern::from_pattern))
        .collect::<Vec<_>>();
    let mut json = serde_json::to_string_pretty(&ast).expect("AST json should serialize");
    json.push('\n');
    json
}

fn render_sexp(parsed: &[ParsedInput]) -> String {
    let mut sections = parsed
        .iter()
        .map(|input| render_document_sexp(&input.source, &input.tree))
        .collect::<Vec<_>>()
        .join("\n");
    if !sections.is_empty() {
        sections.push('\n');
    }
    sections
}

fn render_summary(parsed: &[ParsedInput]) -> String {
    let mut lines = Vec::new();

    for input in parsed {
        let summary = count_summary(&input.source, &input.tree);
        if parsed.len() > 1 {
            lines.push(format!("{}:", input.file_name));
        }
        lines.push(format!("nodes: {}", summary.nodes));
        lines.push(format!("relationships: {}", summary.relationships));
        lines.push(format!("annotations: {}", summary.annotations));
        lines.push(format!("walks: {}", summary.walks));
        if parsed.len() > 1 {
            lines.push(String::new());
        }
    }

    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }

    if lines.is_empty() {
        String::new()
    } else {
        lines.join("\n") + "\n"
    }
}

fn render_document_sexp(source: &str, tree: &Pattern<SyntaxNode>) -> String {
    let mut lines = vec!["(gram_pattern".to_string()];
    if let Some(subject) = tree.value.subject.as_ref() {
        lines.extend(render_record_field_lines(
            source,
            "root",
            subject_subject_span(source, &tree.value.span, subject),
            subject,
            1,
        ));
    }
    for element in &tree.elements {
        lines.extend(render_pattern_lines(source, element, 1));
    }
    lines.push(")".to_string());
    collapse_closing_lines(lines).join("\n")
}

fn render_pattern_lines(source: &str, node: &Pattern<SyntaxNode>, indent: usize) -> Vec<String> {
    match &node.value.kind {
        SyntaxKind::Comment => vec![format!("{}(comment)", spaces(indent))],
        SyntaxKind::Node => {
            if is_pattern_reference(source, node) {
                render_pattern_reference_lines(source, node, indent)
            } else {
                render_subject_container_lines(
                    source,
                    "node_pattern",
                    node_subject_text(source, node),
                    node.value.subject.as_ref(),
                    indent,
                )
            }
        }
        SyntaxKind::Subject => {
            let mut lines = render_subject_container_lines(
                source,
                "subject_pattern",
                subject_pattern_subject_text(source, node),
                node.value.subject.as_ref(),
                indent,
            );
            if !node.elements.is_empty() {
                if let Some(last) = lines.last_mut() {
                    last.pop();
                }
                lines.extend(render_subject_elements_lines(source, node, indent + 1));
                lines.push(format!("{})", spaces(indent)));
            }
            lines
        }
        SyntaxKind::Relationship(kind) => render_relationship_lines(source, node, kind, indent),
        SyntaxKind::Annotated => render_annotated_lines(source, node, indent),
        SyntaxKind::Document => Vec::new(),
    }
}

fn render_subject_container_lines(
    source: &str,
    kind: &str,
    raw_subject: Option<&str>,
    subject: Option<&Subject>,
    indent: usize,
) -> Vec<String> {
    let field_lines = render_subject_field_lines(source, raw_subject, subject, indent + 1);
    if field_lines.is_empty() {
        vec![format!("{}({kind})", spaces(indent))]
    } else {
        let mut lines = vec![format!("{}({kind}", spaces(indent))];
        lines.extend(field_lines);
        lines.push(format!("{})", spaces(indent)));
        lines
    }
}

fn render_pattern_reference_lines(
    source: &str,
    node: &Pattern<SyntaxNode>,
    indent: usize,
) -> Vec<String> {
    let identifier = source_map::span_text(source, &node.value.span).trim();
    let kind = token_kind(identifier);
    vec![
        format!("{}(pattern_reference", spaces(indent)),
        format!("{}identifier: {}", spaces(indent + 1), render_token(kind)),
        format!("{})", spaces(indent)),
    ]
}

fn render_relationship_lines(
    source: &str,
    node: &Pattern<SyntaxNode>,
    arrow_kind: &ArrowKind,
    indent: usize,
) -> Vec<String> {
    let mut lines = vec![format!("{}(relationship_pattern", spaces(indent))];
    if let Some(left) = node.elements.first() {
        lines.extend(prefix_first(
            render_pattern_lines(source, left, indent + 2),
            format!("{}left: ", spaces(indent + 1)),
        ));
    }
    lines.extend(render_arrow_kind_lines(
        source,
        node,
        arrow_kind,
        indent + 1,
    ));
    if let Some(right) = node.elements.get(1) {
        lines.extend(prefix_first(
            render_pattern_lines(source, right, indent + 2),
            format!("{}right: ", spaces(indent + 1)),
        ));
    }
    lines.push(format!("{})", spaces(indent)));
    lines
}

fn render_arrow_kind_lines(
    source: &str,
    node: &Pattern<SyntaxNode>,
    arrow_kind: &ArrowKind,
    indent: usize,
) -> Vec<String> {
    let arrow_name = match arrow_kind {
        ArrowKind::Right => "right_arrow",
        ArrowKind::Left => "left_arrow",
        ArrowKind::Bidirectional => "bidirectional_arrow",
        ArrowKind::Undirected => "undirected_arrow",
    };
    let arrow_subject = arrow_subject_text(source, node);
    let field_lines = render_subject_field_lines(
        source,
        arrow_subject,
        node.value.subject.as_ref(),
        indent + 1,
    );
    if field_lines.is_empty() {
        vec![format!("{}kind: ({arrow_name})", spaces(indent))]
    } else {
        let mut lines = vec![format!("{}kind: ({arrow_name}", spaces(indent))];
        lines.extend(field_lines);
        lines.push(format!("{})", spaces(indent)));
        lines
    }
}

fn render_annotated_lines(source: &str, node: &Pattern<SyntaxNode>, indent: usize) -> Vec<String> {
    let mut lines = vec![format!("{}(annotated_pattern", spaces(indent))];
    lines.extend(render_annotations_lines(
        &node.value.annotations,
        indent + 1,
    ));
    if let Some(inner) = node.elements.first() {
        lines.extend(prefix_first(
            render_pattern_lines(source, inner, indent + 2),
            format!("{}elements: ", spaces(indent + 1)),
        ));
    }
    lines.push(format!("{})", spaces(indent)));
    lines
}

fn render_annotations_lines(annotations: &[Annotation], indent: usize) -> Vec<String> {
    let mut lines = vec![format!("{}annotations: (annotations", spaces(indent))];
    for annotation in annotations {
        match annotation {
            Annotation::Property { key, value } => {
                lines.extend(render_property_annotation_lines(key, value, indent + 1));
            }
            Annotation::Identified { identity, labels } => {
                lines.extend(render_identified_annotation_lines(
                    identity.as_ref(),
                    labels,
                    indent + 1,
                ));
            }
        }
    }
    lines.push(format!("{})", spaces(indent)));
    lines
}

fn render_property_annotation_lines(key: &str, value: &GramValue, indent: usize) -> Vec<String> {
    let mut lines = vec![format!("{}(property_annotation", spaces(indent))];
    lines.push(format!(
        "{}key: {}",
        spaces(indent + 1),
        render_identifier_like(key)
    ));
    lines.extend(prefix_first(
        render_gram_value_lines(value, indent + 2),
        format!("{}value: ", spaces(indent + 1)),
    ));
    lines.push(format!("{})", spaces(indent)));
    lines
}

fn render_identified_annotation_lines(
    identity: Option<&pattern_core::Symbol>,
    labels: &[String],
    indent: usize,
) -> Vec<String> {
    let mut lines = vec![format!("{}(identified_annotation", spaces(indent))];
    if let Some(identity) = identity {
        lines.push(format!(
            "{}identifier: {}",
            spaces(indent + 1),
            render_identifier_like(&identity.0)
        ));
    }
    if !labels.is_empty() {
        lines.extend(render_labels_lines(labels, indent + 1));
    }
    lines.push(format!("{})", spaces(indent)));
    lines
}

fn render_subject_elements_lines(
    source: &str,
    node: &Pattern<SyntaxNode>,
    indent: usize,
) -> Vec<String> {
    let mut lines = vec![format!(
        "{}elements: (subject_pattern_elements",
        spaces(indent)
    )];
    for element in &node.elements {
        lines.extend(render_pattern_lines(source, element, indent + 1));
    }
    lines.push(format!("{})", spaces(indent)));
    lines
}

fn render_record_field_lines(
    _source: &str,
    field_name: &str,
    raw_subject: Option<&str>,
    subject: &Subject,
    indent: usize,
) -> Vec<String> {
    prefix_first(
        render_record_lines(
            subject.properties.iter().collect::<Vec<_>>(),
            raw_subject,
            indent + 1,
        ),
        format!("{}{}: ", spaces(indent), field_name),
    )
}

fn render_record_lines(
    mut properties: Vec<(&String, &pattern_core::Value)>,
    _raw_subject: Option<&str>,
    indent: usize,
) -> Vec<String> {
    properties.sort_by(|left, right| left.0.cmp(right.0));
    let mut lines = vec![format!("{}(record", spaces(indent))];
    for (key, value) in properties {
        lines.extend(render_record_property_lines(key, value, indent + 1));
    }
    lines.push(format!("{})", spaces(indent)));
    lines
}

fn render_record_property_lines(
    key: &str,
    value: &pattern_core::Value,
    indent: usize,
) -> Vec<String> {
    let mut lines = vec![format!("{}(record_property", spaces(indent))];
    lines.push(format!(
        "{}key: {}",
        spaces(indent + 1),
        render_identifier_like(key)
    ));
    lines.extend(prefix_first(
        render_value_lines(value, indent + 2),
        format!("{}value: ", spaces(indent + 1)),
    ));
    lines.push(format!("{})", spaces(indent)));
    lines
}

fn render_value_lines(value: &pattern_core::Value, indent: usize) -> Vec<String> {
    match value {
        pattern_core::Value::VString(_) | pattern_core::Value::VSymbol(_) => vec![
            format!("{}(string_literal", spaces(indent)),
            format!("{}content: (string_content)", spaces(indent + 1)),
            format!("{})", spaces(indent)),
        ],
        pattern_core::Value::VInteger(_) => vec![format!("{}(integer)", spaces(indent))],
        pattern_core::Value::VDecimal(_) => vec![format!("{}(decimal)", spaces(indent))],
        pattern_core::Value::VBoolean(_) => vec![format!("{}(boolean_literal)", spaces(indent))],
        pattern_core::Value::VArray(values) => {
            let mut lines = vec![format!("{}(array", spaces(indent))];
            for value in values {
                lines.extend(render_value_lines(value, indent + 1));
            }
            lines.push(format!("{})", spaces(indent)));
            lines
        }
        pattern_core::Value::VMap(map) => {
            let mut entries = map.iter().collect::<Vec<_>>();
            entries.sort_by(|left, right| left.0.cmp(right.0));
            let mut lines = vec![format!("{}(map", spaces(indent))];
            for (key, value) in entries {
                lines.extend(render_record_property_lines(key, value, indent + 1));
            }
            lines.push(format!("{})", spaces(indent)));
            lines
        }
        pattern_core::Value::VRange(range) => {
            let mut lines = vec![format!("{}(range", spaces(indent))];
            if let Some(lower) = range.lower {
                lines.push(format!(
                    "{}lower: {}",
                    spaces(indent + 1),
                    render_number_token(lower)
                ));
            }
            if let Some(upper) = range.upper {
                lines.push(format!(
                    "{}upper: {}",
                    spaces(indent + 1),
                    render_number_token(upper)
                ));
            }
            lines.push(format!("{})", spaces(indent)));
            lines
        }
        pattern_core::Value::VMeasurement { .. } => {
            vec![format!("{}(measurement)", spaces(indent))]
        }
        pattern_core::Value::VTaggedString { .. } => vec![
            format!("{}(tagged_string", spaces(indent)),
            format!("{}content: (string_content)", spaces(indent + 1)),
            format!("{})", spaces(indent)),
        ],
    }
}

fn render_gram_value_lines(value: &GramValue, indent: usize) -> Vec<String> {
    match value {
        GramValue::String(_) => vec![
            format!("{}(string_literal", spaces(indent)),
            format!("{}content: (string_content)", spaces(indent + 1)),
            format!("{})", spaces(indent)),
        ],
        GramValue::Integer(_) => vec![format!("{}(integer)", spaces(indent))],
        GramValue::Decimal(_) => vec![format!("{}(decimal)", spaces(indent))],
        GramValue::Boolean(_) => vec![format!("{}(boolean_literal)", spaces(indent))],
        GramValue::Array(values) => {
            let mut lines = vec![format!("{}(array", spaces(indent))];
            for value in values {
                lines.extend(render_gram_value_lines(value, indent + 1));
            }
            lines.push(format!("{})", spaces(indent)));
            lines
        }
        GramValue::Range { lower, upper } => {
            let mut lines = vec![format!("{}(range", spaces(indent))];
            lines.push(format!(
                "{}lower: {}",
                spaces(indent + 1),
                render_number_token(*lower as f64)
            ));
            lines.push(format!(
                "{}upper: {}",
                spaces(indent + 1),
                render_number_token(*upper as f64)
            ));
            lines.push(format!("{})", spaces(indent)));
            lines
        }
        GramValue::TaggedString { .. } => vec![
            format!("{}(tagged_string", spaces(indent)),
            format!("{}content: (string_content)", spaces(indent + 1)),
            format!("{})", spaces(indent)),
        ],
    }
}

fn render_subject_field_lines(
    source: &str,
    raw_subject: Option<&str>,
    subject: Option<&Subject>,
    indent: usize,
) -> Vec<String> {
    let Some(subject) = subject else {
        return Vec::new();
    };

    let mut lines = Vec::new();
    if !subject.identity.0.is_empty() {
        let raw_identifier = raw_subject.and_then(scan_raw_identifier);
        lines.push(format!(
            "{}identifier: {}",
            spaces(indent),
            render_token(raw_identifier.map(token_kind).unwrap_or(TokenKind::Symbol))
        ));
    }
    if !subject.labels.is_empty() {
        let mut labels = subject.labels.iter().cloned().collect::<Vec<_>>();
        labels.sort();
        lines.extend(render_labels_lines(&labels, indent));
    }
    if !subject.properties.is_empty() {
        lines.extend(render_record_field_lines(
            source,
            "record",
            raw_subject,
            subject,
            indent,
        ));
    }
    lines
}

fn render_labels_lines(labels: &[String], indent: usize) -> Vec<String> {
    let mut lines = vec![format!("{}labels: (labels", spaces(indent))];
    for _ in labels {
        lines.push(format!("{}(symbol)", spaces(indent + 1)));
    }
    lines.push(format!("{})", spaces(indent)));
    lines
}

fn is_pattern_reference(source: &str, node: &Pattern<SyntaxNode>) -> bool {
    matches!(node.value.kind, SyntaxKind::Node)
        && source_map::span_text(source, &node.value.span)
            .trim()
            .starts_with(|ch| ch != '(')
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

fn arrow_subject_text<'a>(source: &'a str, node: &'a Pattern<SyntaxNode>) -> Option<&'a str> {
    let Some(left) = node.elements.first() else {
        return None;
    };
    let Some(right) = node.elements.get(1) else {
        return None;
    };
    let start = left.value.span.end.min(source.len());
    let end = right.value.span.start.min(source.len());
    if start >= end {
        return None;
    }
    let between = &source[start..end];
    let bracket_start = between.find('[')?;
    let bracket_end = between.rfind(']')?;
    Some(between[bracket_start + 1..bracket_end].trim())
}

fn subject_subject_span<'a>(
    source: &'a str,
    span: &'a SourceSpan,
    _subject: &Subject,
) -> Option<&'a str> {
    Some(source_map::span_text(source, span).trim())
}

#[derive(Clone, Copy)]
enum TokenKind {
    Symbol,
    StringLiteral,
    Integer,
}

fn scan_raw_identifier(raw_subject: &str) -> Option<&str> {
    let trimmed = raw_subject.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.starts_with('{') {
        return None;
    }

    let bytes = trimmed.as_bytes();
    if bytes.first() == Some(&b'"') || bytes.first() == Some(&b'\'') || bytes.first() == Some(&b'`')
    {
        let quote = bytes[0];
        let mut index = 1usize;
        while index < bytes.len() {
            if bytes[index] == quote && bytes.get(index.wrapping_sub(1)) != Some(&b'\\') {
                return Some(&trimmed[..=index]);
            }
            index += 1;
        }
        return Some(trimmed);
    }

    let mut end = 0usize;
    while end < bytes.len() {
        let ch = bytes[end] as char;
        if ch.is_ascii_whitespace() || matches!(ch, ':' | '{') {
            break;
        }
        end += 1;
    }

    (end > 0).then_some(&trimmed[..end])
}

fn token_kind(raw: &str) -> TokenKind {
    let trimmed = raw.trim();
    if trimmed.starts_with('"') || trimmed.starts_with('\'') || trimmed.starts_with('`') {
        TokenKind::StringLiteral
    } else if trimmed.parse::<i64>().is_ok() {
        TokenKind::Integer
    } else {
        TokenKind::Symbol
    }
}

fn render_identifier_like(value: &str) -> String {
    render_token(token_kind(value))
}

fn render_number_token(value: f64) -> String {
    if value.fract() == 0.0 {
        "(integer)".to_string()
    } else {
        "(decimal)".to_string()
    }
}

fn render_token(kind: TokenKind) -> String {
    match kind {
        TokenKind::Symbol => "(symbol)".to_string(),
        TokenKind::Integer => "(integer)".to_string(),
        TokenKind::StringLiteral => "(string_literal\n      content: (string_content))".to_string(),
    }
}

#[derive(Default)]
struct SummaryCounts {
    nodes: usize,
    relationships: usize,
    annotations: usize,
    walks: usize,
}

fn count_summary(source: &str, tree: &Pattern<SyntaxNode>) -> SummaryCounts {
    let mut counts = SummaryCounts::default();
    count_summary_inner(source, tree, &mut counts);
    counts
}

fn count_summary_inner(source: &str, node: &Pattern<SyntaxNode>, counts: &mut SummaryCounts) {
    match &node.value.kind {
        SyntaxKind::Document | SyntaxKind::Comment => {}
        SyntaxKind::Node => {
            if !is_pattern_reference(source, node) {
                counts.nodes += 1;
            }
        }
        SyntaxKind::Relationship(_) => {
            counts.relationships += 1;
            if node
                .elements
                .get(1)
                .is_some_and(|right| matches!(right.value.kind, SyntaxKind::Relationship(_)))
            {
                counts.walks += 1;
            }
        }
        SyntaxKind::Subject => {}
        SyntaxKind::Annotated => {
            counts.annotations += node.value.annotations.len();
        }
    }

    for child in &node.elements {
        count_summary_inner(source, child, counts);
    }
}

fn prefix_first(mut lines: Vec<String>, prefix: String) -> Vec<String> {
    if let Some(first) = lines.first_mut() {
        *first = format!("{prefix}{}", first.trim_start());
    }
    lines
}

fn spaces(indent: usize) -> String {
    "  ".repeat(indent)
}

fn collapse_closing_lines(lines: Vec<String>) -> Vec<String> {
    let mut collapsed: Vec<String> = Vec::new();
    for line in lines {
        if line.trim() == ")" {
            if let Some(previous) = collapsed.last_mut() {
                previous.push(')');
            } else {
                collapsed.push(line);
            }
        } else {
            collapsed.push(line);
        }
    }
    collapsed
}

fn read_stdin() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(input)
}
