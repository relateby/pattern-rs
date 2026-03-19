use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
        }
    }

    pub fn exit_code(self) -> i32 {
        match self {
            Severity::Info => 0,
            Severity::Warning => 1,
            Severity::Error => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemediationGrade {
    Auto,
    Guided,
    Ambiguous,
    None,
}

impl RemediationGrade {
    pub fn as_str(self) -> &'static str {
        match self {
            RemediationGrade::Auto => "auto",
            RemediationGrade::Guided => "guided",
            RemediationGrade::Ambiguous => "ambiguous",
            RemediationGrade::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticCode {
    P001,
    P002,
    P003,
    P004,
    P005,
    P006,
    P007,
    P008,
}

impl DiagnosticCode {
    pub fn as_str(self) -> &'static str {
        match self {
            DiagnosticCode::P001 => "P001",
            DiagnosticCode::P002 => "P002",
            DiagnosticCode::P003 => "P003",
            DiagnosticCode::P004 => "P004",
            DiagnosticCode::P005 => "P005",
            DiagnosticCode::P006 => "P006",
            DiagnosticCode::P007 => "P007",
            DiagnosticCode::P008 => "P008",
        }
    }

    pub fn rule_name(self) -> &'static str {
        match self {
            DiagnosticCode::P001 => "parse-failure",
            DiagnosticCode::P002 => "no-duplicate-identity",
            DiagnosticCode::P003 => "no-duplicate-annotation-key",
            DiagnosticCode::P004 => "label-case",
            DiagnosticCode::P005 => "dangling-reference",
            DiagnosticCode::P006 => "empty-array",
            DiagnosticCode::P007 => "no-schema",
            DiagnosticCode::P008 => "unknown-document-kind",
        }
    }

    pub fn severity(self) -> Severity {
        match self {
            DiagnosticCode::P001 | DiagnosticCode::P002 | DiagnosticCode::P003 => Severity::Error,
            DiagnosticCode::P004 | DiagnosticCode::P005 | DiagnosticCode::P008 => Severity::Warning,
            DiagnosticCode::P006 | DiagnosticCode::P007 => Severity::Info,
        }
    }

    pub fn grade(self) -> RemediationGrade {
        match self {
            DiagnosticCode::P004 => RemediationGrade::Auto,
            DiagnosticCode::P001
            | DiagnosticCode::P002
            | DiagnosticCode::P003
            | DiagnosticCode::P006
            | DiagnosticCode::P008 => RemediationGrade::Guided,
            DiagnosticCode::P005 => RemediationGrade::Ambiguous,
            DiagnosticCode::P007 => RemediationGrade::None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl Location {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Edit {
    Replace {
        file: PathBuf,
        line: u32,
        column: u32,
        replace: String,
        with: String,
    },
    DeleteLine {
        file: PathBuf,
        line: u32,
    },
    Append {
        file: PathBuf,
        content: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemediationSteps {
    Inline(Vec<String>),
    Structured(Vec<Edit>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemediationOption {
    pub description: String,
    pub edit: Edit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Remediation {
    Auto {
        summary: String,
        steps: RemediationSteps,
    },
    Guided {
        summary: String,
        steps: RemediationSteps,
    },
    Ambiguous {
        summary: String,
        decision: String,
        options: Vec<RemediationOption>,
    },
    None,
}

impl Remediation {
    pub fn grade(&self) -> RemediationGrade {
        match self {
            Remediation::Auto { .. } => RemediationGrade::Auto,
            Remediation::Guided { .. } => RemediationGrade::Guided,
            Remediation::Ambiguous { .. } => RemediationGrade::Ambiguous,
            Remediation::None => RemediationGrade::None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub rule: &'static str,
    pub message: String,
    pub location: Location,
    pub remediation: Remediation,
}

impl Diagnostic {
    pub fn new(
        code: DiagnosticCode,
        message: impl Into<String>,
        location: Location,
        remediation: Remediation,
    ) -> Self {
        Self {
            severity: code.severity(),
            rule: code.rule_name(),
            code,
            message: message.into(),
            location,
            remediation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileDiagnostics {
    pub file: String,
    pub diagnostics: Vec<Diagnostic>,
}

impl FileDiagnostics {
    pub fn new(file: impl Into<String>, diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            file: file.into(),
            diagnostics,
        }
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == Severity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == Severity::Warning)
            .count()
    }

    pub fn auto_fixable_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.remediation.grade() == RemediationGrade::Auto)
            .count()
    }
}

pub fn highest_severity(reports: &[FileDiagnostics]) -> Option<Severity> {
    reports
        .iter()
        .flat_map(|report| {
            report
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.severity)
        })
        .max()
}

pub fn exit_code_for_reports(reports: &[FileDiagnostics]) -> i32 {
    highest_severity(reports).map_or(0, Severity::exit_code)
}
