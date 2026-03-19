use crate::cli::OutputFormatArg;
use crate::diagnostics::{FileDiagnostics, Remediation, Severity};
use std::io::{self, IsTerminal};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputFormat {
    Gram,
    Text,
    Json,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(value: OutputFormatArg) -> Self {
        match value {
            OutputFormatArg::Gram => OutputFormat::Gram,
            OutputFormatArg::Text => OutputFormat::Text,
            OutputFormatArg::Json => OutputFormat::Json,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutputContext {
    pub format: OutputFormat,
    pub use_color: bool,
}

impl OutputContext {
    pub fn new(format: OutputFormat) -> Self {
        Self {
            format,
            use_color: io::stdout().is_terminal(),
        }
    }
}

pub fn render_text_reports(reports: &[FileDiagnostics], use_color: bool) -> String {
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
                diagnostic.message()
            ));

            if let Some(template) = diagnostic.remediation_template() {
                lines.push(format!(
                    "    remediation {} ({}) {}",
                    template.id,
                    diagnostic.remediation.grade().as_str(),
                    template.summary
                ));
            }

            if let Remediation::Ambiguous { options, .. } = &diagnostic.remediation {
                for option in options {
                    lines.push(format!(
                        "    option {} {}",
                        option.id,
                        option.summary(diagnostic.code)
                    ));
                }
            }
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
