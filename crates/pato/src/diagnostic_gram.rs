use crate::diagnostics::{
    Diagnostic, DiagnosticCode, Edit, FileDiagnostics, Remediation, RemediationGrade,
    RemediationOption, RemediationSteps, Severity,
};
use crate::output::OutputFormat;
use gram_codec::to_gram_with_header;
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
        OutputFormat::Text => writer.write_all(to_text(reports, use_color).as_bytes()),
    }
}

pub fn to_gram(reports: &[FileDiagnostics]) -> io::Result<String> {
    let header = header_record(if reports.len() == 1 {
        Some(reports[0].file.as_str())
    } else {
        None
    });
    let patterns = if reports.len() == 1 {
        single_file_patterns(&reports[0])
    } else {
        vec![run_pattern(reports)]
    };

    to_gram_with_header(header, &patterns).map_err(|error: gram_codec::SerializeError| {
        io::Error::new(io::ErrorKind::Other, error.to_string())
    })
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
                "locations".to_string(),
                JsonValue::Array(report.diagnostics.iter().map(diagnostic_to_json).collect()),
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

pub fn to_text(reports: &[FileDiagnostics], use_color: bool) -> String {
    let mut lines = Vec::new();
    for report in reports {
        if reports.len() > 1 {
            lines.push(format!("{}:", report.file));
        }
        if report.diagnostics.is_empty() {
            lines.push("  clean".to_string());
            continue;
        }
        for diagnostic in &report.diagnostics {
            let severity = style_severity(diagnostic.severity, use_color);
            lines.push(format!(
                "  {} {} {}:{} {}",
                severity,
                diagnostic.code.as_str(),
                diagnostic.location.line,
                diagnostic.location.column,
                diagnostic.message
            ));
        }
    }
    lines.join("\n") + "\n"
}

fn style_severity(severity: Severity, use_color: bool) -> String {
    let label = severity.as_str().to_uppercase();
    if !use_color {
        return label;
    }

    let code = match severity {
        Severity::Error => 31,
        Severity::Warning => 33,
        Severity::Info => 36,
    };
    format!("\x1b[{}m{}\x1b[0m", code, label)
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

fn single_file_patterns(report: &FileDiagnostics) -> Vec<Pattern<Subject>> {
    if report.diagnostics.is_empty() {
        return vec![summary_pattern(report)];
    }

    report
        .diagnostics
        .iter()
        .enumerate()
        .map(|(index, diagnostic)| location_pattern(index + 1, diagnostic))
        .collect()
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
                .map(|(offset, diagnostic)| location_pattern(offset + 1, diagnostic))
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

fn location_pattern(index: usize, diagnostic: &Diagnostic) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert(
        "line".to_string(),
        Value::VInteger(i64::from(diagnostic.location.line)),
    );
    properties.insert(
        "column".to_string(),
        Value::VInteger(i64::from(diagnostic.location.column)),
    );
    Pattern::pattern(
        subject(&format!("loc{index}"), &["Location"], properties),
        vec![diagnostic_pattern(index, diagnostic)],
    )
}

fn diagnostic_pattern(index: usize, diagnostic: &Diagnostic) -> Pattern<Subject> {
    let mut properties = HashMap::new();
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
    properties.insert(
        "message".to_string(),
        Value::VString(diagnostic.message.clone()),
    );

    if let Remediation::Guided {
        steps: RemediationSteps::Inline(steps),
        ..
    } = &diagnostic.remediation
    {
        properties.insert(
            "remediations".to_string(),
            Value::VArray(steps.iter().cloned().map(Value::VString).collect()),
        );
    }

    if let Remediation::Ambiguous { decision, .. } = &diagnostic.remediation {
        properties.insert("decision".to_string(), Value::VString(decision.clone()));
    }

    let children = remediation_children(index, &diagnostic.remediation);
    if children.is_empty() {
        Pattern::point(subject(&format!("d{index}"), &["Diagnostic"], properties))
    } else {
        Pattern::pattern(
            subject(&format!("d{index}"), &["Diagnostic"], properties),
            children,
        )
    }
}

fn remediation_children(index: usize, remediation: &Remediation) -> Vec<Pattern<Subject>> {
    match remediation {
        Remediation::Auto { summary, steps } | Remediation::Guided { summary, steps } => {
            match steps {
                RemediationSteps::Inline(_) => Vec::new(),
                RemediationSteps::Structured(edits) => edits
                    .iter()
                    .enumerate()
                    .map(|(offset, edit)| {
                        remediation_pattern(index + offset, remediation.grade(), summary, edit)
                    })
                    .collect(),
            }
        }
        Remediation::Ambiguous { options, .. } => options
            .iter()
            .enumerate()
            .map(|(offset, option)| option_pattern(index + offset, option))
            .collect(),
        Remediation::None => Vec::new(),
    }
}

fn remediation_pattern(
    index: usize,
    grade: RemediationGrade,
    summary: &str,
    edit: &Edit,
) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert(
        "grade".to_string(),
        Value::VString(grade.as_str().to_string()),
    );
    properties.insert("summary".to_string(), Value::VString(summary.to_string()));
    apply_edit_properties(&mut properties, edit);
    Pattern::point(subject(&format!("r{index}"), &["Remediation"], properties))
}

fn option_pattern(index: usize, option: &RemediationOption) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert(
        "description".to_string(),
        Value::VString(option.description.clone()),
    );
    apply_edit_properties(&mut properties, &option.edit);
    Pattern::point(subject(&format!("opt{index}"), &["Option"], properties))
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
            properties.insert("replace".to_string(), Value::VString(replace.clone()));
            properties.insert("with".to_string(), Value::VString(with.clone()));
            properties.insert("line".to_string(), Value::VInteger(i64::from(*line)));
            properties.insert("column".to_string(), Value::VInteger(i64::from(*column)));
        }
        Edit::DeleteLine { line, .. } => {
            properties.insert("delete_line".to_string(), Value::VInteger(i64::from(*line)));
        }
        Edit::Append { content, .. } => {
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

fn diagnostic_to_json(diagnostic: &Diagnostic) -> JsonValue {
    let mut object = JsonMap::new();
    object.insert(
        "location".to_string(),
        json!({
            "line": diagnostic.location.line,
            "column": diagnostic.location.column
        }),
    );
    object.insert(
        "diagnostic".to_string(),
        json!({
            "severity": diagnostic.severity.as_str(),
            "code": diagnostic.code.as_str(),
            "rule": diagnostic.rule,
            "message": diagnostic.message,
        }),
    );

    match &diagnostic.remediation {
        Remediation::Auto { summary, steps } | Remediation::Guided { summary, steps } => {
            object.insert(
                "remediation".to_string(),
                json!({
                    "grade": diagnostic.remediation.grade().as_str(),
                    "summary": summary,
                    "steps": steps_to_json(steps),
                }),
            );
        }
        Remediation::Ambiguous {
            summary,
            decision,
            options,
        } => {
            object.insert(
                "remediation".to_string(),
                json!({
                    "grade": "ambiguous",
                    "summary": summary,
                    "decision": decision,
                    "options": options.iter().map(option_to_json).collect::<Vec<_>>(),
                }),
            );
        }
        Remediation::None => {
            object.insert(
                "remediation".to_string(),
                json!({
                    "grade": "none"
                }),
            );
        }
    }

    JsonValue::Object(object)
}

fn file_report_to_json(report: &FileDiagnostics) -> JsonValue {
    json!({
        "file": report.file,
        "errors": report.error_count(),
        "warnings": report.warning_count(),
        "locations": report.diagnostics.iter().map(diagnostic_to_json).collect::<Vec<_>>(),
    })
}

fn steps_to_json(steps: &RemediationSteps) -> JsonValue {
    match steps {
        RemediationSteps::Inline(lines) => {
            JsonValue::Array(lines.iter().map(|line| json!(line)).collect())
        }
        RemediationSteps::Structured(edits) => {
            JsonValue::Array(edits.iter().map(edit_to_json).collect())
        }
    }
}

fn option_to_json(option: &RemediationOption) -> JsonValue {
    json!({
        "description": option.description,
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
            "line": line,
        }),
        Edit::Append { file, content } => json!({
            "kind": "append",
            "file": file,
            "content": content,
        }),
    }
}

#[allow(dead_code)]
fn _assert_grade_mapping() {
    let _ = DiagnosticCode::P001.grade();
}
