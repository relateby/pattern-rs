use crate::diagnostics::{
    exit_code_for_reports, rule_info, Diagnostic, DiagnosticCode, Edit, FileDiagnostics, Location,
    Remediation, RemediationOption,
};
use crate::{editor, source_map};
use gram_codec::cst::{Annotation, CstParseResult, SourceSpan, SyntaxKind, SyntaxNode};
use gram_codec::{lower, parse_gram_cst, Pattern, Subject};
use pattern_core::Value;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use strsim::levenshtein;

pub struct LintOutcome {
    pub reports: Vec<FileDiagnostics>,
    pub had_io_error: bool,
}

impl LintOutcome {
    pub fn exit_code(&self) -> i32 {
        if self.had_io_error {
            3
        } else {
            exit_code_for_reports(&self.reports)
        }
    }
}

pub fn lint_paths(files: &[PathBuf], fix: bool) -> LintOutcome {
    let mut reports = Vec::new();
    let mut had_io_error = false;

    for file in files {
        if file == Path::new("-") {
            match read_stdin() {
                Ok(source) => reports.push(lint_source("<stdin>", &source, None, fix)),
                Err(error) => {
                    had_io_error = true;
                    eprintln!("failed to read stdin: {error}");
                }
            }
            continue;
        }

        match fs::read_to_string(file) {
            Ok(source) => reports.push(lint_source(
                &file.display().to_string(),
                &source,
                Some(file),
                fix,
            )),
            Err(error) => {
                had_io_error = true;
                eprintln!("failed to read {}: {error}", file.display());
            }
        }
    }

    LintOutcome {
        reports,
        had_io_error,
    }
}

pub fn lint_source(
    file_name: &str,
    source: &str,
    path: Option<&Path>,
    fix: bool,
) -> FileDiagnostics {
    let report = analyze_source(file_name, source, path);
    if !fix {
        return report;
    }

    let Some(path) = path else {
        return report;
    };

    let has_ambiguous = report
        .diagnostics
        .iter()
        .any(|diagnostic| matches!(diagnostic.remediation, Remediation::Ambiguous { .. }));
    if has_ambiguous {
        return report;
    }

    let edits: Vec<Edit> = report
        .diagnostics
        .iter()
        .flat_map(|diagnostic| match &diagnostic.remediation {
            Remediation::Auto { edits, .. } => edits.clone(),
            _ => Vec::new(),
        })
        .collect();

    if edits.is_empty() {
        return report;
    }

    if let Err(error) = editor::apply_edits(path, &edits) {
        eprintln!("failed to apply fixes to {}: {error}", path.display());
        return report;
    }

    match fs::read_to_string(path) {
        Ok(rewritten) => analyze_source(file_name, &rewritten, Some(path)),
        Err(error) => {
            eprintln!("failed to verify fixed file {}: {error}", path.display());
            report
        }
    }
}

fn analyze_source(file_name: &str, source: &str, path: Option<&Path>) -> FileDiagnostics {
    let parse_result = parse_gram_cst(source);
    let diagnostics = if !parse_result.is_valid() {
        parse_error_diagnostics(source, path, &parse_result)
    } else if let Err(error) = gram_codec::parse_gram(source) {
        vec![nom_parse_error_diagnostic(path, error)]
    } else {
        let patterns = lower(parse_result.tree.clone());
        collect_diagnostics(file_name, source, path, &parse_result.tree, &patterns)
    };

    FileDiagnostics::new(file_name, diagnostics)
}

fn parse_error_diagnostics(
    source: &str,
    _path: Option<&Path>,
    parse_result: &CstParseResult,
) -> Vec<Diagnostic> {
    let error_spans = if parse_result.errors.is_empty() {
        vec![SourceSpan {
            start: 0,
            end: source.len(),
        }]
    } else {
        let mut spans = parse_result.errors.clone();
        spans.sort_by(source_map::compare_spans);
        spans
    };

    error_spans
        .into_iter()
        .map(|span| {
            let location = source_map::span_start_location(source, &span);
            let snippet = source_map::span_text(source, &span).trim();
            let remediation_id = DiagnosticCode::P001
                .remediation_id()
                .expect("P001 should have a remediation template");
            let diagnostic = Diagnostic::new(
                DiagnosticCode::P001,
                location,
                Remediation::Guided {
                    id: remediation_id,
                    edits: Vec::new(),
                },
            );
            if snippet.is_empty() {
                diagnostic
            } else {
                diagnostic.with_fact("snippet", snippet.to_string())
            }
        })
        .collect()
}

fn nom_parse_error_diagnostic(path: Option<&Path>, error: gram_codec::ParseError) -> Diagnostic {
    let location = error
        .location()
        .map(|location| Location::new(location.line as u32, location.column as u32))
        .unwrap_or(Location::new(1, 1));
    let _ = path;
    Diagnostic::new(
        DiagnosticCode::P001,
        location,
        Remediation::Guided {
            id: DiagnosticCode::P001
                .remediation_id()
                .expect("P001 should have a remediation template"),
            edits: Vec::new(),
        },
    )
    .with_fact("detail", error.to_string())
}

fn collect_diagnostics(
    file_name: &str,
    source: &str,
    path: Option<&Path>,
    tree: &Pattern<SyntaxNode>,
    patterns: &[Pattern<Subject>],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(check_duplicate_identities(source, path, tree));
    diagnostics.extend(check_duplicate_annotation_keys(source, path, tree));
    diagnostics.extend(check_label_case(source, path, tree));
    diagnostics.extend(check_dangling_references(source, path, tree));
    diagnostics.extend(check_empty_arrays(source, path, patterns));
    diagnostics.extend(check_document_kind(file_name, source, path, tree));
    diagnostics.sort_by(|left, right| compare_locations(left.location, right.location));
    diagnostics
}

fn check_duplicate_identities(
    source: &str,
    _path: Option<&Path>,
    tree: &Pattern<SyntaxNode>,
) -> Vec<Diagnostic> {
    let occurrences = collect_identity_occurrences(source, tree);
    let mut first_seen: HashMap<String, Location> = HashMap::new();
    let mut diagnostics = Vec::new();

    for occurrence in occurrences {
        if occurrence.identity.is_empty() || !occurrence.is_definition {
            continue;
        }
        if let Some(first_location) = first_seen.get(&occurrence.identity) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::P002,
                    occurrence.location,
                    Remediation::Guided {
                        id: DiagnosticCode::P002
                            .remediation_id()
                            .expect("P002 should have a remediation template"),
                        edits: Vec::new(),
                    },
                )
                .with_fact("identity", occurrence.identity.clone())
                .with_fact("first_line", first_location.line)
                .with_fact("first_column", first_location.column),
            );
        } else {
            first_seen.insert(occurrence.identity.clone(), occurrence.location);
        }
    }

    diagnostics
}

fn check_duplicate_annotation_keys(
    source: &str,
    path: Option<&Path>,
    tree: &Pattern<SyntaxNode>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    collect_duplicate_annotation_keys_inner(source, path, tree, &mut diagnostics);
    diagnostics
}

fn check_label_case(
    source: &str,
    path: Option<&Path>,
    tree: &Pattern<SyntaxNode>,
) -> Vec<Diagnostic> {
    collect_label_occurrences(source, tree)
        .into_iter()
        .filter_map(|occurrence| {
            let expected = if occurrence.is_relationship {
                occurrence.label.to_uppercase()
            } else {
                title_case(&occurrence.label)
            };
            if occurrence.label == expected {
                return None;
            }

            let file = path?.to_path_buf();
            let observed = occurrence.label.clone();
            let expected = expected.clone();
            Some(
                Diagnostic::new(
                    DiagnosticCode::P004,
                    occurrence.location,
                    Remediation::Auto {
                        id: DiagnosticCode::P004
                            .remediation_id()
                            .expect("P004 should have a remediation template"),
                        edits: vec![Edit::Replace {
                            file,
                            line: occurrence.location.line,
                            column: occurrence.location.column,
                            replace: observed.clone(),
                            with: expected.clone(),
                        }],
                    },
                )
                .with_fact(
                    "label_kind",
                    if occurrence.is_relationship {
                        "relationship"
                    } else {
                        "node"
                    },
                )
                .with_fact("observed", observed)
                .with_fact("expected", expected),
            )
        })
        .collect()
}

fn check_dangling_references(
    source: &str,
    path: Option<&Path>,
    tree: &Pattern<SyntaxNode>,
) -> Vec<Diagnostic> {
    let occurrences = collect_identity_occurrences(source, tree);
    let defined: HashSet<String> = occurrences
        .iter()
        .filter(|occurrence| occurrence.is_definition)
        .map(|occurrence| occurrence.identity.clone())
        .collect();

    occurrences
        .into_iter()
        .filter(|occurrence| occurrence.is_reference && !defined.contains(&occurrence.identity))
        .map(|occurrence| {
            let suggested = nearest_identity(&occurrence.identity, &defined)
                .unwrap_or_else(|| occurrence.identity.clone());
            let file = path.map(Path::to_path_buf).unwrap_or_default();
            let remediation = rule_info(DiagnosticCode::P005)
                .remediations
                .first()
                .expect("P005 should have a remediation template");
            Diagnostic::new(
                DiagnosticCode::P005,
                occurrence.location,
                Remediation::Ambiguous {
                    id: remediation.id,
                    options: vec![
                        RemediationOption {
                            id: remediation.option_templates[0].id,
                            edit: Edit::Replace {
                                file: file.clone(),
                                line: occurrence.location.line,
                                column: occurrence.location.column,
                                replace: occurrence.identity.clone(),
                                with: suggested.clone(),
                            },
                        },
                        RemediationOption {
                            id: remediation.option_templates[1].id,
                            edit: Edit::Append {
                                file,
                                content: format!("({}:Entity)", occurrence.identity),
                            },
                        },
                    ],
                },
            )
            .with_fact("unresolved_identity", occurrence.identity)
            .with_fact("suggested_identity", suggested)
        })
        .collect()
}

fn check_empty_arrays(
    source: &str,
    path: Option<&Path>,
    patterns: &[Pattern<Subject>],
) -> Vec<Diagnostic> {
    let mut array_locations = find_all_occurrences(source, "[]");
    let mut diagnostics = Vec::new();

    for pattern in patterns {
        walk_property_values(&pattern.value.properties, &mut |value| {
            if matches!(value, Value::VArray(items) if items.is_empty()) {
                let location = if array_locations.is_empty() {
                    Location::new(1, 1)
                } else {
                    array_locations.remove(0)
                };
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::P006,
                    location,
                    Remediation::Guided {
                        id: DiagnosticCode::P006
                            .remediation_id()
                            .expect("P006 should have a remediation template"),
                        edits: Vec::new(),
                    },
                ));
            }
        });
        collect_empty_arrays_from_children(
            &pattern.elements,
            &mut array_locations,
            path,
            &mut diagnostics,
        );
    }

    diagnostics
}

fn collect_empty_arrays_from_children(
    patterns: &[Pattern<Subject>],
    array_locations: &mut Vec<Location>,
    _path: Option<&Path>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for pattern in patterns {
        walk_property_values(&pattern.value.properties, &mut |value| {
            if matches!(value, Value::VArray(items) if items.is_empty()) {
                let location = if array_locations.is_empty() {
                    Location::new(1, 1)
                } else {
                    array_locations.remove(0)
                };
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::P006,
                    location,
                    Remediation::Guided {
                        id: DiagnosticCode::P006
                            .remediation_id()
                            .expect("P006 should have a remediation template"),
                        edits: Vec::new(),
                    },
                ));
            }
        });
        collect_empty_arrays_from_children(&pattern.elements, array_locations, _path, diagnostics);
    }
}

fn check_document_kind(
    _file_name: &str,
    source: &str,
    _path: Option<&Path>,
    tree: &Pattern<SyntaxNode>,
) -> Vec<Diagnostic> {
    let Some(header) = tree.value.subject.as_ref() else {
        return Vec::new();
    };
    let Some(Value::VString(kind)) = header.properties.get("kind") else {
        return Vec::new();
    };
    if matches!(kind.as_str(), "diagnostics" | "rule") {
        return Vec::new();
    }

    let location = document_kind_location(source, tree, kind).unwrap_or(Location::new(1, 1));
    vec![Diagnostic::new(
        DiagnosticCode::P008,
        location,
        Remediation::Guided {
            id: DiagnosticCode::P008
                .remediation_id()
                .expect("P008 should have a remediation template"),
            edits: Vec::new(),
        },
    )
    .with_fact("kind", kind.clone())]
}

fn read_stdin() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(input)
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

fn nearest_identity(target: &str, defined: &HashSet<String>) -> Option<String> {
    let mut candidates: Vec<_> = defined
        .iter()
        .map(|identity| (identity.clone(), levenshtein(target, identity)))
        .collect();
    candidates.sort_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(&right.0)));
    candidates.into_iter().next().map(|candidate| candidate.0)
}

fn walk_property_values(properties: &HashMap<String, Value>, callback: &mut impl FnMut(&Value)) {
    for value in properties.values() {
        callback(value);
        walk_nested_values(value, callback);
    }
}

fn walk_nested_values(value: &Value, callback: &mut impl FnMut(&Value)) {
    match value {
        Value::VArray(items) => {
            for item in items {
                callback(item);
                walk_nested_values(item, callback);
            }
        }
        Value::VMap(entries) => {
            for item in entries.values() {
                callback(item);
                walk_nested_values(item, callback);
            }
        }
        _ => {}
    }
}

#[derive(Clone)]
struct IdentityOccurrence {
    identity: String,
    location: Location,
    is_definition: bool,
    is_reference: bool,
}

fn collect_identity_occurrences(
    source: &str,
    tree: &Pattern<SyntaxNode>,
) -> Vec<IdentityOccurrence> {
    let mut occurrences = Vec::new();
    collect_identity_occurrences_inner(source, tree, false, &mut occurrences);
    occurrences
}

fn collect_identity_occurrences_inner(
    source: &str,
    pattern: &Pattern<SyntaxNode>,
    top_level: bool,
    occurrences: &mut Vec<IdentityOccurrence>,
) {
    match pattern.value.kind {
        SyntaxKind::Document => {
            for child in &pattern.elements {
                collect_identity_occurrences_inner(source, child, true, occurrences);
            }
            return;
        }
        SyntaxKind::Comment => return,
        SyntaxKind::Annotated => {
            for child in &pattern.elements {
                collect_identity_occurrences_inner(source, child, top_level, occurrences);
            }
            return;
        }
        _ => {}
    }

    if let Some(subject) = pattern.value.subject.as_ref() {
        if !subject.identity.0.is_empty() {
            let location = subject_identity_location(source, pattern, &subject.identity.0)
                .unwrap_or_else(|| source_map::span_start_location(source, &pattern.value.span));
            let is_definition = top_level
                || !subject.labels.is_empty()
                || !subject.properties.is_empty()
                || !pattern.elements.is_empty();
            let is_reference = !top_level
                && subject.labels.is_empty()
                && subject.properties.is_empty()
                && pattern.elements.is_empty()
                && matches!(pattern.value.kind, SyntaxKind::Node);
            occurrences.push(IdentityOccurrence {
                identity: subject.identity.0.clone(),
                location,
                is_definition,
                is_reference,
            });
        }
    }

    for child in &pattern.elements {
        collect_identity_occurrences_inner(source, child, false, occurrences);
    }
}

#[derive(Clone)]
struct LabelOccurrence {
    label: String,
    location: Location,
    is_relationship: bool,
}

fn collect_label_occurrences(source: &str, tree: &Pattern<SyntaxNode>) -> Vec<LabelOccurrence> {
    let mut occurrences = Vec::new();
    collect_label_occurrences_inner(source, tree, &mut occurrences);
    occurrences
}

fn collect_label_occurrences_inner(
    source: &str,
    pattern: &Pattern<SyntaxNode>,
    occurrences: &mut Vec<LabelOccurrence>,
) {
    match pattern.value.kind {
        SyntaxKind::Document | SyntaxKind::Comment => {}
        SyntaxKind::Annotated => {
            for child in &pattern.elements {
                collect_label_occurrences_inner(source, child, occurrences);
            }
            return;
        }
        SyntaxKind::Node | SyntaxKind::Subject | SyntaxKind::Relationship(_) => {
            if let Some(subject) = pattern.value.subject.as_ref() {
                for (label, location) in subject_label_locations(source, pattern, subject) {
                    occurrences.push(LabelOccurrence {
                        label,
                        location,
                        is_relationship: matches!(pattern.value.kind, SyntaxKind::Relationship(_)),
                    });
                }
            }
        }
    }

    for child in &pattern.elements {
        collect_label_occurrences_inner(source, child, occurrences);
    }
}

fn collect_duplicate_annotation_keys_inner(
    source: &str,
    _path: Option<&Path>,
    node: &Pattern<SyntaxNode>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if matches!(node.value.kind, SyntaxKind::Annotated) {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for annotation in &node.value.annotations {
            if let Annotation::Property { key, .. } = annotation {
                let occurrence_index = counts.entry(key.clone()).or_insert(0);
                *occurrence_index += 1;
                if *occurrence_index >= 2 {
                    let location =
                        nth_property_annotation_location(source, node, key, *occurrence_index - 1)
                            .unwrap_or_else(|| {
                                source_map::span_start_location(source, &node.value.span)
                            });
                    diagnostics.push(
                        Diagnostic::new(
                            DiagnosticCode::P003,
                            location,
                            Remediation::Guided {
                                id: DiagnosticCode::P003
                                    .remediation_id()
                                    .expect("P003 should have a remediation template"),
                                edits: Vec::new(),
                            },
                        )
                        .with_fact("key", key.clone()),
                    );
                }
            }
        }
    }

    for child in &node.elements {
        collect_duplicate_annotation_keys_inner(source, _path, child, diagnostics);
    }
}

fn subject_identity_location(
    source: &str,
    node: &Pattern<SyntaxNode>,
    identity: &str,
) -> Option<Location> {
    let (base_offset, region) = subject_region(source, node)?;
    find_identifier_in_region(region, identity)
        .map(|relative| source_map::offset_to_location(source, base_offset + relative))
}

fn subject_label_locations(
    source: &str,
    node: &Pattern<SyntaxNode>,
    subject: &Subject,
) -> Vec<(String, Location)> {
    let Some((base_offset, region)) = subject_region(source, node) else {
        return Vec::new();
    };

    scan_labels(region)
        .into_iter()
        .filter(|(label, _)| subject.labels.contains(label))
        .map(|(label, relative)| {
            (
                label,
                source_map::offset_to_location(source, base_offset + relative),
            )
        })
        .collect()
}

fn subject_region<'a>(source: &'a str, node: &'a Pattern<SyntaxNode>) -> Option<(usize, &'a str)> {
    let fragment = source_map::span_text(source, &node.value.span);
    match node.value.kind {
        SyntaxKind::Node => Some((node.value.span.start, fragment)),
        SyntaxKind::Subject => {
            let split = fragment.find('|').unwrap_or(fragment.len());
            Some((node.value.span.start, &fragment[..split]))
        }
        SyntaxKind::Relationship(_) => {
            let open = fragment.find('[')?;
            let close = fragment[open + 1..].find(']')?;
            let start = open + 1;
            let end = open + 1 + close;
            Some((node.value.span.start + start, &fragment[start..end]))
        }
        _ => None,
    }
}

fn find_identifier_in_region(region: &str, token: &str) -> Option<usize> {
    region.match_indices(token).find_map(|(offset, _)| {
        identifier_boundaries(region, offset, token.len()).then_some(offset)
    })
}

fn scan_labels(region: &str) -> Vec<(String, usize)> {
    let bytes = region.as_bytes();
    let mut cursor = 0;
    let mut labels = Vec::new();

    while cursor < bytes.len() {
        if bytes[cursor] != b':' {
            cursor += 1;
            continue;
        }

        let label_start = cursor + 1;
        let mut end = label_start;
        while end < bytes.len() && is_ident_char(bytes[end] as char) {
            end += 1;
        }

        if end > label_start {
            labels.push((region[label_start..end].to_string(), label_start));
            cursor = end;
        } else {
            cursor += 1;
        }
    }

    labels
}

fn nth_property_annotation_location(
    source: &str,
    node: &Pattern<SyntaxNode>,
    key: &str,
    occurrence_index: usize,
) -> Option<Location> {
    let fragment = source_map::span_text(source, &node.value.span);
    let bytes = fragment.as_bytes();
    let mut cursor = 0;
    let mut seen = 0;

    while cursor < bytes.len() {
        if bytes[cursor] != b'@' || bytes.get(cursor + 1) == Some(&b'@') {
            cursor += 1;
            continue;
        }

        let key_start = cursor + 1;
        if !fragment[key_start..].starts_with(key) {
            cursor += 1;
            continue;
        }

        let key_end = key_start + key.len();
        let next = bytes.get(key_end).copied().map(char::from);
        if next.is_some_and(is_ident_char) {
            cursor += 1;
            continue;
        }

        if seen == occurrence_index {
            return Some(source_map::location_at_span_offset(
                source,
                &node.value.span,
                key_start,
            ));
        }

        seen += 1;
        cursor = key_end;
    }

    None
}

fn document_kind_location(
    source: &str,
    tree: &Pattern<SyntaxNode>,
    kind: &str,
) -> Option<Location> {
    let header_end = tree
        .elements
        .first()
        .map(|element| element.value.span.start)
        .unwrap_or(source.len());
    let header = &source[..header_end.min(source.len())];
    header
        .match_indices(kind)
        .next()
        .map(|(offset, _)| source_map::offset_to_location(source, offset))
}

fn find_all_occurrences(source: &str, needle: &str) -> Vec<Location> {
    source
        .match_indices(needle)
        .map(|(offset, _)| source_map::offset_to_location(source, offset))
        .collect()
}

fn compare_locations(left: Location, right: Location) -> Ordering {
    (left.line, left.column).cmp(&(right.line, right.column))
}

fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-')
}

fn identifier_boundaries(source: &str, offset: usize, len: usize) -> bool {
    let before = source[..offset].chars().next_back();
    let after = source[offset + len..].chars().next();
    before.map_or(true, |ch| !is_ident_char(ch)) && after.map_or(true, |ch| !is_ident_char(ch))
}
