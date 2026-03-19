use crate::commands::fmt;
use crate::diagnostics::{Diagnostic, Edit, FactValue, FileDiagnostics, Remediation};
use crate::output::{render_text_reports, OutputFormat};
use gram_codec::{to_gram_pattern, to_gram_with_header};
use pattern_core::{Pattern, Subject, Value};
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

const PATO_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn render_diagnostics<W: Write>(
    reports: &[FileDiagnostics],
    output_format: OutputFormat,
    writer: &mut W,
    use_color: bool,
) -> io::Result<()> {
    match output_format {
        OutputFormat::Gram => writer.write_all(to_gram(reports)?.as_bytes()),
        OutputFormat::Json => {
            serde_json::to_writer_pretty(&mut *writer, &to_json(reports)?)
                .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;
            writer.write_all(b"\n")
        }
        OutputFormat::Text => writer.write_all(render_text_reports(reports, use_color).as_bytes()),
    }
}

pub fn to_gram(reports: &[FileDiagnostics]) -> io::Result<String> {
    to_gram_internal(reports, true)
}

pub fn to_gram_without_comments(reports: &[FileDiagnostics]) -> io::Result<String> {
    to_gram_internal(reports, false)
}

fn to_gram_internal(reports: &[FileDiagnostics], include_comments: bool) -> io::Result<String> {
    let header = header_record(if reports.len() == 1 {
        Some(reports[0].file.as_str())
    } else {
        None
    });

    if reports.len() != 1 {
        let raw = to_gram_with_header(header, &[run_pattern(reports)]).map_err(serialize_error)?;
        return fmt::format_gram(&raw);
    }

    let report = &reports[0];
    if report.diagnostics.is_empty() {
        let raw =
            to_gram_with_header(header, &[summary_pattern(report)]).map_err(serialize_error)?;
        return fmt::format_gram(&raw);
    }

    let mut sections = vec![to_gram_with_header(header, &[]).map_err(serialize_error)?];
    for (index, diagnostic) in report.diagnostics.iter().enumerate() {
        let mut block = String::new();
        if include_comments {
            for comment in diagnostic.comments() {
                block.push_str("// ");
                block.push_str(&comment);
                block.push('\n');
            }
        }
        block.push_str(
            &to_gram_pattern(&problem_pattern(index + 1, diagnostic)).map_err(serialize_error)?,
        );
        sections.push(block);
    }

    fmt::format_gram(&sections.join("\n\n"))
}

pub fn to_json(reports: &[FileDiagnostics]) -> io::Result<JsonValue> {
    if reports.len() == 1 {
        let report = &reports[0];
        let mut object = JsonMap::new();
        object.insert("kind".to_string(), json!("diagnostics"));
        object.insert("patoVersion".to_string(), json!(PATO_VERSION));
        object.insert("file".to_string(), json!(report.file));
        if report.diagnostics.is_empty() {
            object.insert(
                "summary".to_string(),
                json!({
                    "errors": 0,
                    "warnings": 0,
                    "autoFixable": 0
                }),
            );
        } else {
            object.insert(
                "problems".to_string(),
                JsonValue::Array(report.diagnostics.iter().map(problem_to_json).collect()),
            );
        }
        Ok(JsonValue::Object(object))
    } else {
        Ok(json!({
            "kind": "diagnostics",
            "patoVersion": PATO_VERSION,
            "run": {
                "command": "lint",
                "files": reports.iter().map(file_report_to_json).collect::<Vec<_>>()
            }
        }))
    }
}

fn serialize_error(error: gram_codec::SerializeError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, error.to_string())
}

fn header_record(file: Option<&str>) -> HashMap<String, Value> {
    let mut record = HashMap::new();
    record.insert(
        "kind".to_string(),
        Value::VString("diagnostics".to_string()),
    );
    record.insert(
        "pato_version".to_string(),
        Value::VString(PATO_VERSION.to_string()),
    );
    if let Some(file) = file {
        record.insert("file".to_string(), Value::VString(file.to_string()));
    }
    record
}

fn run_pattern(reports: &[FileDiagnostics]) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert("command".to_string(), Value::VString("lint".to_string()));
    Pattern::pattern(
        subject("run", &["Run"], properties),
        reports
            .iter()
            .enumerate()
            .map(|(index, report)| file_pattern(index + 1, report))
            .collect(),
    )
}

fn file_pattern(index: usize, report: &FileDiagnostics) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert("file".to_string(), Value::VString(report.file.clone()));
    properties.insert(
        "errors".to_string(),
        Value::VInteger(report.error_count() as i64),
    );
    properties.insert(
        "warnings".to_string(),
        Value::VInteger(report.warning_count() as i64),
    );
    Pattern::pattern(
        subject(&format!("f{index}"), &["FileResult"], properties),
        if report.diagnostics.is_empty() {
            vec![summary_pattern(report)]
        } else {
            report
                .diagnostics
                .iter()
                .enumerate()
                .map(|(offset, diagnostic)| problem_pattern(offset + 1, diagnostic))
                .collect()
        },
    )
}

fn summary_pattern(report: &FileDiagnostics) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert(
        "errors".to_string(),
        Value::VInteger(report.error_count() as i64),
    );
    properties.insert(
        "warnings".to_string(),
        Value::VInteger(report.warning_count() as i64),
    );
    properties.insert(
        "auto_fixable".to_string(),
        Value::VInteger(report.auto_fixable_count() as i64),
    );
    Pattern::point(subject("summary", &["Summary"], properties))
}

fn problem_pattern(index: usize, diagnostic: &Diagnostic) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert(
        "line".to_string(),
        Value::VInteger(i64::from(diagnostic.location.line)),
    );
    properties.insert(
        "column".to_string(),
        Value::VInteger(i64::from(diagnostic.location.column)),
    );
    properties.insert(
        "severity".to_string(),
        Value::VString(diagnostic.severity.as_str().to_string()),
    );
    properties.insert(
        "code".to_string(),
        Value::VString(diagnostic.code.as_str().to_string()),
    );
    properties.insert(
        "rule".to_string(),
        Value::VString(diagnostic.rule.to_string()),
    );
    if let Some(remediation_id) = diagnostic.remediation.id() {
        properties.insert(
            "remediation".to_string(),
            Value::VString(remediation_id.to_string()),
        );
    }
    for (key, value) in &diagnostic.facts {
        properties.insert(key.clone(), fact_value_to_gram(value));
    }

    let children = remediation_children(index, diagnostic);
    if children.is_empty() {
        Pattern::point(subject(
            &format!("problem{index}"),
            &["Problem"],
            properties,
        ))
    } else {
        Pattern::pattern(
            subject(&format!("problem{index}"), &["Problem"], properties),
            children,
        )
    }
}

fn remediation_children(index: usize, diagnostic: &Diagnostic) -> Vec<Pattern<Subject>> {
    match &diagnostic.remediation {
        Remediation::Auto { edits, .. } | Remediation::Guided { edits, .. } => edits
            .iter()
            .enumerate()
            .map(|(offset, edit)| apply_pattern(index, offset + 1, edit))
            .collect(),
        Remediation::Ambiguous { options, .. } => options
            .iter()
            .enumerate()
            .map(|(offset, option)| option_pattern(index, offset + 1, option))
            .collect(),
        Remediation::None => Vec::new(),
    }
}

fn apply_pattern(problem_index: usize, child_index: usize, edit: &Edit) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    apply_edit_properties(&mut properties, edit);
    Pattern::point(subject(
        &format!("apply{problem_index}_{child_index}"),
        &["Apply"],
        properties,
    ))
}

fn option_pattern(
    problem_index: usize,
    child_index: usize,
    option: &crate::diagnostics::RemediationOption,
) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert("id".to_string(), Value::VString(option.id.to_string()));
    apply_edit_properties(&mut properties, &option.edit);
    Pattern::point(subject(
        &format!("option{problem_index}_{child_index}"),
        &["Option"],
        properties,
    ))
}

fn apply_edit_properties(properties: &mut HashMap<String, Value>, edit: &Edit) {
    match edit {
        Edit::Replace {
            line,
            column,
            replace,
            with,
            ..
        } => {
            properties.insert("kind".to_string(), Value::VString("replace".to_string()));
            properties.insert("replace".to_string(), Value::VString(replace.clone()));
            properties.insert("with".to_string(), Value::VString(with.clone()));
            properties.insert("line".to_string(), Value::VInteger(i64::from(*line)));
            properties.insert("column".to_string(), Value::VInteger(i64::from(*column)));
        }
        Edit::DeleteLine { line, .. } => {
            properties.insert("kind".to_string(), Value::VString("deleteLine".to_string()));
            properties.insert("delete_line".to_string(), Value::VInteger(i64::from(*line)));
        }
        Edit::Append { content, .. } => {
            properties.insert("kind".to_string(), Value::VString("append".to_string()));
            properties.insert("append".to_string(), Value::VString(content.clone()));
        }
    }
}

fn subject(identity: &str, labels: &[&str], properties: HashMap<String, Value>) -> Subject {
    Subject {
        identity: pattern_core::Symbol(identity.to_string()),
        labels: labels
            .iter()
            .map(|label| (*label).to_string())
            .collect::<HashSet<_>>(),
        properties,
    }
}

fn fact_value_to_gram(value: &FactValue) -> Value {
    match value {
        FactValue::String(value) => Value::VString(value.clone()),
        FactValue::Integer(value) => Value::VInteger(*value),
        FactValue::Boolean(value) => Value::VBoolean(*value),
    }
}

fn problem_to_json(diagnostic: &Diagnostic) -> JsonValue {
    let mut object = JsonMap::new();
    object.insert("line".to_string(), json!(diagnostic.location.line));
    object.insert("column".to_string(), json!(diagnostic.location.column));
    object.insert("severity".to_string(), json!(diagnostic.severity.as_str()));
    object.insert("code".to_string(), json!(diagnostic.code.as_str()));
    object.insert("rule".to_string(), json!(diagnostic.rule));
    if let Some(remediation_id) = diagnostic.remediation.id() {
        object.insert("remediation".to_string(), json!(remediation_id));
    }

    let mut facts = JsonMap::new();
    for (key, value) in &diagnostic.facts {
        facts.insert(key.clone(), fact_value_to_json(value));
    }
    if !facts.is_empty() {
        object.insert("facts".to_string(), JsonValue::Object(facts));
    }

    match &diagnostic.remediation {
        Remediation::Auto { edits, .. } | Remediation::Guided { edits, .. } => {
            if !edits.is_empty() {
                object.insert(
                    "apply".to_string(),
                    JsonValue::Array(edits.iter().map(edit_to_json).collect()),
                );
            }
        }
        Remediation::Ambiguous { options, .. } => {
            object.insert(
                "options".to_string(),
                JsonValue::Array(options.iter().map(option_to_json).collect()),
            );
        }
        Remediation::None => {}
    }

    JsonValue::Object(object)
}

fn file_report_to_json(report: &FileDiagnostics) -> JsonValue {
    json!({
        "file": report.file,
        "errors": report.error_count(),
        "warnings": report.warning_count(),
        "problems": report.diagnostics.iter().map(problem_to_json).collect::<Vec<_>>(),
    })
}

fn fact_value_to_json(value: &FactValue) -> JsonValue {
    match value {
        FactValue::String(value) => json!(value),
        FactValue::Integer(value) => json!(value),
        FactValue::Boolean(value) => json!(value),
    }
}

fn option_to_json(option: &crate::diagnostics::RemediationOption) -> JsonValue {
    json!({
        "id": option.id,
        "edit": edit_to_json(&option.edit),
    })
}

fn edit_to_json(edit: &Edit) -> JsonValue {
    match edit {
        Edit::Replace {
            file,
            line,
            column,
            replace,
            with,
        } => json!({
            "kind": "replace",
            "file": file,
            "line": line,
            "column": column,
            "replace": replace,
            "with": with,
        }),
        Edit::DeleteLine { file, line } => json!({
            "kind": "deleteLine",
            "file": file,
            "delete_line": line,
        }),
        Edit::Append { file, content } => json!({
            "kind": "append",
            "file": file,
            "content": content,
        }),
    }
}
