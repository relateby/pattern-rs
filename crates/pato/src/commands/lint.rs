use crate::diagnostics::{
    exit_code_for_reports, Diagnostic, DiagnosticCode, Edit, FileDiagnostics, Location,
    Remediation, RemediationOption, RemediationSteps,
};
use crate::editor;
use pattern_core::{Pattern, Subject, Value};
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
            Remediation::Auto {
                steps: RemediationSteps::Structured(edits),
                ..
            } => edits.clone(),
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
    let parse_result = gram_codec::parse_gram(source);
    let diagnostics = match parse_result {
        Ok(patterns) => collect_diagnostics(file_name, source, path, &patterns),
        Err(error) => vec![parse_error_diagnostic(path, error)],
    };

    FileDiagnostics::new(file_name, diagnostics)
}

fn parse_error_diagnostic(path: Option<&Path>, error: gram_codec::ParseError) -> Diagnostic {
    let location = error
        .location()
        .map(|location| Location::new(location.line as u32, location.column as u32))
        .unwrap_or(Location::new(1, 1));
    Diagnostic::new(
        DiagnosticCode::P001,
        error.to_string(),
        location,
        Remediation::Guided {
            summary: "Fix the gram syntax and re-run lint".to_string(),
            steps: RemediationSteps::Inline(vec![
                "Correct the syntax near the reported location.".to_string(),
                "Re-run `pato lint` to confirm the file parses cleanly.".to_string(),
            ]),
        },
    )
    .with_default_file(path)
}

fn collect_diagnostics(
    file_name: &str,
    source: &str,
    path: Option<&Path>,
    patterns: &[Pattern<Subject>],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(check_duplicate_identities(source, path, patterns));
    diagnostics.extend(check_duplicate_annotation_keys(source, path));
    diagnostics.extend(check_label_case(source, path, patterns));
    diagnostics.extend(check_dangling_references(source, path, patterns));
    diagnostics.extend(check_empty_arrays(source, path, patterns));
    diagnostics.extend(check_document_kind(file_name, source, path));
    diagnostics.sort_by(|left, right| compare_locations(left.location, right.location));
    diagnostics
}

fn check_duplicate_identities(
    source: &str,
    path: Option<&Path>,
    patterns: &[Pattern<Subject>],
) -> Vec<Diagnostic> {
    let occurrences = collect_identity_occurrences(source, patterns);
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
                    format!(
                        "Identity '{}' is defined twice: first at {}:{}, again here",
                        occurrence.identity, first_location.line, first_location.column
                    ),
                    occurrence.location,
                    Remediation::Guided {
                        summary: "Rename one of the duplicate identities".to_string(),
                        steps: RemediationSteps::Inline(vec![format!(
                            "Rename either the first or second '{}' identity so both definitions are unique.",
                            occurrence.identity
                        )]),
                    },
                )
                .with_default_file(path),
            );
        } else {
            first_seen.insert(occurrence.identity.clone(), occurrence.location);
        }
    }

    diagnostics
}

fn check_duplicate_annotation_keys(source: &str, path: Option<&Path>) -> Vec<Diagnostic> {
    find_duplicate_annotation_keys(source)
        .into_iter()
        .map(|duplicate| {
            Diagnostic::new(
                DiagnosticCode::P003,
                format!(
                    "Annotation key '{}' appears more than once before the same pattern",
                    duplicate.key
                ),
                duplicate.location,
                Remediation::Guided {
                    summary: format!("Remove the duplicate @{} annotation", duplicate.key),
                    steps: RemediationSteps::Inline(vec![format!(
                        "Keep only one @{} annotation in the annotation chain.",
                        duplicate.key
                    )]),
                },
            )
            .with_default_file(path)
        })
        .collect()
}

fn check_label_case(
    source: &str,
    path: Option<&Path>,
    patterns: &[Pattern<Subject>],
) -> Vec<Diagnostic> {
    collect_label_occurrences(source, patterns)
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
            let summary = format!("Recase to {expected}");
            Some(
                Diagnostic::new(
                    DiagnosticCode::P004,
                    format!(
                        "{} label '{}' should be {}",
                        if occurrence.is_relationship {
                            "Relationship"
                        } else {
                            "Node"
                        },
                        occurrence.label,
                        if occurrence.is_relationship {
                            "uppercase"
                        } else {
                            "TitleCase"
                        }
                    ),
                    occurrence.location,
                    Remediation::Auto {
                        summary,
                        steps: RemediationSteps::Structured(vec![Edit::Replace {
                            file,
                            line: occurrence.location.line,
                            column: occurrence.location.column,
                            replace: occurrence.label,
                            with: expected,
                        }]),
                    },
                )
                .with_default_file(path),
            )
        })
        .collect()
}

fn check_dangling_references(
    source: &str,
    path: Option<&Path>,
    patterns: &[Pattern<Subject>],
) -> Vec<Diagnostic> {
    let occurrences = collect_identity_occurrences(source, patterns);
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
            Diagnostic::new(
                DiagnosticCode::P005,
                format!(
                    "'{}' is referenced but not defined in this file",
                    occurrence.identity
                ),
                occurrence.location,
                Remediation::Ambiguous {
                    summary: format!("Choose whether '{}' should be renamed or defined", occurrence.identity),
                    decision: format!(
                        "Is '{}' a misspelling of an existing identity, or should it be defined here?",
                        occurrence.identity
                    ),
                    options: vec![
                        RemediationOption {
                            description: format!(
                                "Rename reference to '{}' (closest match)",
                                suggested
                            ),
                            edit: Edit::Replace {
                                file: file.clone(),
                                line: occurrence.location.line,
                                column: occurrence.location.column,
                                replace: occurrence.identity.clone(),
                                with: suggested,
                            },
                        },
                        RemediationOption {
                            description: format!("Add a '{}' definition to this file", occurrence.identity),
                            edit: Edit::Append {
                                file,
                                content: format!("({}:Entity)", occurrence.identity),
                            },
                        },
                    ],
                },
            )
            .with_default_file(path)
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
                diagnostics.push(
                    Diagnostic::new(
                        DiagnosticCode::P006,
                        "Empty array values are discouraged in gram files",
                        location,
                        Remediation::Guided {
                            summary: "Replace the empty array with a concrete value or remove the property".to_string(),
                            steps: RemediationSteps::Inline(vec![
                                "Remove the empty array property if it is unused.".to_string(),
                                "Otherwise replace it with a non-empty array or another concrete value.".to_string(),
                            ]),
                        },
                    )
                    .with_default_file(path),
                );
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
    path: Option<&Path>,
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
                diagnostics.push(
                    Diagnostic::new(
                        DiagnosticCode::P006,
                        "Empty array values are discouraged in gram files",
                        location,
                        Remediation::Guided {
                            summary: "Replace the empty array with a concrete value or remove the property".to_string(),
                            steps: RemediationSteps::Inline(vec![
                                "Remove the empty array property if it is unused.".to_string(),
                                "Otherwise replace it with a non-empty array or another concrete value.".to_string(),
                            ]),
                        },
                    )
                    .with_default_file(path),
                );
            }
        });
        collect_empty_arrays_from_children(&pattern.elements, array_locations, path, diagnostics);
    }
}

fn check_document_kind(file_name: &str, source: &str, path: Option<&Path>) -> Vec<Diagnostic> {
    let Ok((header, _)) = gram_codec::parse_gram_with_header(source) else {
        return Vec::new();
    };
    let Some(header) = header else {
        return Vec::new();
    };
    let Some(Value::VString(kind)) = header.get("kind") else {
        return Vec::new();
    };
    if matches!(kind.as_str(), "diagnostics" | "rule") {
        return Vec::new();
    }

    let location = find_identifier_literal(source, kind).unwrap_or(Location::new(1, 1));
    vec![Diagnostic::new(
        DiagnosticCode::P008,
        format!(
            "Document header kind '{}' is not recognized in {}",
            kind, file_name
        ),
        location,
        Remediation::Guided {
            summary: "Use a recognized document kind or remove the header".to_string(),
            steps: RemediationSteps::Inline(vec![
                "Change the kind to 'diagnostics' or 'rule' if this is pato output.".to_string(),
                "Otherwise remove the kind property from the document header.".to_string(),
            ]),
        },
    )
    .with_default_file(path)]
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
    patterns: &[Pattern<Subject>],
) -> Vec<IdentityOccurrence> {
    let mut locator = CursorLocator::new(source);
    let mut occurrences = Vec::new();
    for pattern in patterns {
        collect_identity_occurrences_inner(pattern, true, &mut locator, &mut occurrences);
    }
    occurrences
}

fn collect_identity_occurrences_inner(
    pattern: &Pattern<Subject>,
    top_level: bool,
    locator: &mut CursorLocator<'_>,
    occurrences: &mut Vec<IdentityOccurrence>,
) {
    if !pattern.value.identity.0.is_empty() {
        let location = locator
            .find_identifier(&pattern.value.identity.0)
            .unwrap_or(Location::new(1, 1));
        let is_definition = top_level
            || !pattern.value.labels.is_empty()
            || !pattern.value.properties.is_empty()
            || !pattern.elements.is_empty();
        let is_reference = !top_level
            && pattern.value.labels.is_empty()
            && pattern.value.properties.is_empty()
            && pattern.elements.is_empty();
        occurrences.push(IdentityOccurrence {
            identity: pattern.value.identity.0.clone(),
            location,
            is_definition,
            is_reference,
        });
    }

    for child in &pattern.elements {
        collect_identity_occurrences_inner(child, false, locator, occurrences);
    }
}

#[derive(Clone)]
struct LabelOccurrence {
    label: String,
    location: Location,
    is_relationship: bool,
}

fn collect_label_occurrences(source: &str, patterns: &[Pattern<Subject>]) -> Vec<LabelOccurrence> {
    let mut locator = CursorLocator::new(source);
    let mut occurrences = Vec::new();
    for pattern in patterns {
        collect_label_occurrences_inner(pattern, &mut locator, &mut occurrences);
    }
    occurrences
}

fn collect_label_occurrences_inner(
    pattern: &Pattern<Subject>,
    locator: &mut CursorLocator<'_>,
    occurrences: &mut Vec<LabelOccurrence>,
) {
    let is_relationship = pattern.elements.len() == 2;
    let mut labels: Vec<String> = pattern.value.labels.iter().cloned().collect();
    labels.sort();
    for label in labels {
        let location = locator.find_label(&label).unwrap_or(Location::new(1, 1));
        occurrences.push(LabelOccurrence {
            label,
            location,
            is_relationship,
        });
    }

    for child in &pattern.elements {
        collect_label_occurrences_inner(child, locator, occurrences);
    }
}

#[derive(Clone)]
struct DuplicateAnnotation {
    key: String,
    location: Location,
}

fn find_duplicate_annotation_keys(source: &str) -> Vec<DuplicateAnnotation> {
    let bytes = source.as_bytes();
    let mut offset = 0;
    let mut duplicates = Vec::new();

    while offset < bytes.len() {
        while offset < bytes.len() && bytes[offset].is_ascii_whitespace() {
            offset += 1;
        }
        if offset >= bytes.len() || bytes[offset] != b'@' {
            offset += 1;
            continue;
        }

        let chain_start = offset;
        let mut seen = HashSet::new();
        loop {
            if offset >= bytes.len() || bytes[offset] != b'@' {
                break;
            }
            let key_start = offset + 1;
            let mut cursor = key_start;
            while cursor < bytes.len() && is_ident_char(bytes[cursor] as char) {
                cursor += 1;
            }
            if cursor == key_start {
                break;
            }
            let key = &source[key_start..cursor];
            if !seen.insert(key.to_string()) {
                duplicates.push(DuplicateAnnotation {
                    key: key.to_string(),
                    location: offset_to_location(source, key_start),
                });
            }
            offset = cursor;
            if offset < bytes.len() && bytes[offset] == b'(' {
                offset += 1;
                while offset < bytes.len() && bytes[offset] != b')' {
                    offset += 1;
                }
                if offset < bytes.len() {
                    offset += 1;
                }
            }
            while offset < bytes.len() && bytes[offset].is_ascii_whitespace() {
                if bytes[offset] == b'\n' {
                    break;
                }
                offset += 1;
            }
            if offset >= bytes.len() || bytes[offset] != b'@' {
                break;
            }
        }

        if chain_start == offset {
            offset += 1;
        }
    }

    duplicates
}

fn find_all_occurrences(source: &str, needle: &str) -> Vec<Location> {
    source
        .match_indices(needle)
        .map(|(offset, _)| offset_to_location(source, offset))
        .collect()
}

fn find_identifier_literal(source: &str, needle: &str) -> Option<Location> {
    source
        .match_indices(needle)
        .map(|(offset, _)| offset_to_location(source, offset))
        .next()
}

fn offset_to_location(source: &str, offset: usize) -> Location {
    let prefix = &source[..offset.min(source.len())];
    let line = prefix.matches('\n').count() as u32 + 1;
    let column = prefix
        .rfind('\n')
        .map(|position| (offset - position) as u32)
        .unwrap_or(offset as u32 + 1);
    Location::new(line, column)
}

fn compare_locations(left: Location, right: Location) -> Ordering {
    (left.line, left.column).cmp(&(right.line, right.column))
}

fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-')
}

struct CursorLocator<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> CursorLocator<'a> {
    fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    fn find_identifier(&mut self, token: &str) -> Option<Location> {
        let (offset, location) = self.find_token(token, false)?;
        self.cursor = offset + token.len();
        Some(location)
    }

    fn find_label(&mut self, label: &str) -> Option<Location> {
        let search = format!(":{label}");
        let (offset, location) = self.find_token(&search, true)?;
        self.cursor = offset + search.len();
        Some(location)
    }

    fn find_token(&self, token: &str, skip_prefix: bool) -> Option<(usize, Location)> {
        let slice = &self.source[self.cursor..];
        for (relative, _) in slice.match_indices(token) {
            let absolute = self.cursor + relative;
            if !skip_prefix && !identifier_boundaries(self.source, absolute, token.len()) {
                continue;
            }
            let location = offset_to_location(self.source, absolute + usize::from(skip_prefix));
            return Some((absolute, location));
        }
        None
    }
}

fn identifier_boundaries(source: &str, offset: usize, len: usize) -> bool {
    let before = source[..offset].chars().next_back();
    let after = source[offset + len..].chars().next();
    before.map_or(true, |ch| !is_ident_char(ch)) && after.map_or(true, |ch| !is_ident_char(ch))
}

trait DiagnosticExt {
    fn with_default_file(self, path: Option<&Path>) -> Self;
}

impl DiagnosticExt for Diagnostic {
    fn with_default_file(self, _path: Option<&Path>) -> Self {
        self
    }
}
