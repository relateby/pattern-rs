use std::collections::BTreeMap;
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
        rule_info(self).name
    }

    pub fn severity(self) -> Severity {
        rule_info(self).severity
    }

    pub fn grade(self) -> RemediationGrade {
        rule_info(self).grade
    }

    pub fn remediation_id(self) -> Option<&'static str> {
        rule_info(self)
            .remediations
            .first()
            .map(|template| template.id)
    }

    pub fn parse(input: &str) -> Option<Self> {
        match input.to_ascii_uppercase().as_str() {
            "P001" => Some(DiagnosticCode::P001),
            "P002" => Some(DiagnosticCode::P002),
            "P003" => Some(DiagnosticCode::P003),
            "P004" => Some(DiagnosticCode::P004),
            "P005" => Some(DiagnosticCode::P005),
            "P006" => Some(DiagnosticCode::P006),
            "P007" => Some(DiagnosticCode::P007),
            "P008" => Some(DiagnosticCode::P008),
            _ => None,
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
pub struct RemediationOption {
    pub id: &'static str,
    pub edit: Edit,
}

impl RemediationOption {
    pub fn summary(&self, code: DiagnosticCode) -> &'static str {
        option_template(code, self.id)
            .map(|template| template.summary)
            .unwrap_or(self.id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Remediation {
    Auto {
        id: &'static str,
        edits: Vec<Edit>,
    },
    Guided {
        id: &'static str,
        edits: Vec<Edit>,
    },
    Ambiguous {
        id: &'static str,
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

    pub fn id(&self) -> Option<&'static str> {
        match self {
            Remediation::Auto { id, .. }
            | Remediation::Guided { id, .. }
            | Remediation::Ambiguous { id, .. } => Some(*id),
            Remediation::None => None,
        }
    }

    pub fn edits(&self) -> &[Edit] {
        match self {
            Remediation::Auto { edits, .. } | Remediation::Guided { edits, .. } => edits,
            Remediation::Ambiguous { .. } | Remediation::None => &[],
        }
    }

    pub fn options(&self) -> &[RemediationOption] {
        match self {
            Remediation::Ambiguous { options, .. } => options,
            Remediation::Auto { .. } | Remediation::Guided { .. } | Remediation::None => &[],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FactValue {
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl FactValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            FactValue::String(value) => Some(value),
            FactValue::Integer(_) | FactValue::Boolean(_) => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            FactValue::Integer(value) => Some(*value),
            FactValue::String(_) | FactValue::Boolean(_) => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            FactValue::Boolean(value) => Some(*value),
            FactValue::String(_) | FactValue::Integer(_) => None,
        }
    }
}

impl From<&str> for FactValue {
    fn from(value: &str) -> Self {
        FactValue::String(value.to_string())
    }
}

impl From<String> for FactValue {
    fn from(value: String) -> Self {
        FactValue::String(value)
    }
}

impl From<i64> for FactValue {
    fn from(value: i64) -> Self {
        FactValue::Integer(value)
    }
}

impl From<u32> for FactValue {
    fn from(value: u32) -> Self {
        FactValue::Integer(i64::from(value))
    }
}

impl From<usize> for FactValue {
    fn from(value: usize) -> Self {
        FactValue::Integer(value as i64)
    }
}

impl From<bool> for FactValue {
    fn from(value: bool) -> Self {
        FactValue::Boolean(value)
    }
}

pub type DiagnosticFacts = BTreeMap<String, FactValue>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub rule: &'static str,
    pub location: Location,
    pub facts: DiagnosticFacts,
    pub remediation: Remediation,
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, location: Location, remediation: Remediation) -> Self {
        Self {
            severity: code.severity(),
            rule: code.rule_name(),
            code,
            location,
            facts: BTreeMap::new(),
            remediation,
        }
    }

    pub fn with_fact(mut self, key: impl Into<String>, value: impl Into<FactValue>) -> Self {
        self.facts.insert(key.into(), value.into());
        self
    }

    pub fn fact_str(&self, key: &str) -> Option<&str> {
        self.facts.get(key).and_then(FactValue::as_str)
    }

    pub fn fact_i64(&self, key: &str) -> Option<i64> {
        self.facts.get(key).and_then(FactValue::as_i64)
    }

    pub fn fact_bool(&self, key: &str) -> Option<bool> {
        self.facts.get(key).and_then(FactValue::as_bool)
    }

    pub fn remediation_template(&self) -> Option<&'static RemediationTemplate> {
        let template = rule_info(self.code).remediations.first()?;
        if self.remediation.id() == Some(template.id) {
            Some(template)
        } else {
            None
        }
    }

    pub fn message(&self) -> String {
        match self.code {
            DiagnosticCode::P001 => {
                if let Some(detail) = self.fact_str("detail").filter(|value| !value.is_empty()) {
                    return detail.to_string();
                }
                if let Some(snippet) = self.fact_str("snippet").filter(|value| !value.is_empty()) {
                    format!("Gram syntax error near '{snippet}'")
                } else {
                    "Gram syntax error near this location".to_string()
                }
            }
            DiagnosticCode::P002 => format!(
                "Identity '{}' is defined twice: first at {}:{}, again here",
                self.fact_str("identity").unwrap_or("<unknown>"),
                self.fact_i64("first_line").unwrap_or(1),
                self.fact_i64("first_column").unwrap_or(1)
            ),
            DiagnosticCode::P003 => format!(
                "Annotation key '{}' appears more than once before the same pattern",
                self.fact_str("key").unwrap_or("<unknown>")
            ),
            DiagnosticCode::P004 => format!(
                "{} label '{}' should be {}",
                match self.fact_str("label_kind").unwrap_or("node") {
                    "relationship" => "Relationship",
                    _ => "Node",
                },
                self.fact_str("observed").unwrap_or("<unknown>"),
                match self.fact_str("label_kind").unwrap_or("node") {
                    "relationship" => "uppercase",
                    _ => "TitleCase",
                }
            ),
            DiagnosticCode::P005 => format!(
                "'{}' is referenced but not defined in this file",
                self.fact_str("unresolved_identity").unwrap_or("<unknown>")
            ),
            DiagnosticCode::P006 => "Empty array values are discouraged in gram files".to_string(),
            DiagnosticCode::P007 => "No schema was found for this file".to_string(),
            DiagnosticCode::P008 => format!(
                "Document header kind '{}' is not recognized",
                self.fact_str("kind").unwrap_or("<unknown>")
            ),
        }
    }

    pub fn comments(&self) -> Vec<String> {
        let mut comments = vec![self.message()];
        let rule = rule_info(self.code);
        comments.push(rule.description.to_string());

        if let Some(template) = self.remediation_template() {
            comments.push(format!(
                "Remediation `{}`: {}",
                template.id, template.summary
            ));
            comments.push(template.details.to_string());

            if let Remediation::Ambiguous { options, .. } = &self.remediation {
                for option in options {
                    comments.push(format!(
                        "Option `{}`: {}",
                        option.id,
                        option.summary(self.code)
                    ));
                }
            }
        }

        comments
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemediationOptionTemplate {
    pub id: &'static str,
    pub summary: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemediationParameter {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemediationTemplate {
    pub id: &'static str,
    pub summary: &'static str,
    pub details: &'static str,
    pub parameters: &'static [RemediationParameter],
    pub option_templates: &'static [RemediationOptionTemplate],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleInfo {
    pub code: DiagnosticCode,
    pub name: &'static str,
    pub severity: Severity,
    pub grade: RemediationGrade,
    pub description: &'static str,
    pub remediations: &'static [RemediationTemplate],
    pub trigger_example_gram: &'static str,
}

const NO_OPTIONS: &[RemediationOptionTemplate] = &[];
const NO_PARAMETERS: &[RemediationParameter] = &[];
const P001_PARAMETERS: &[RemediationParameter] = &[
    RemediationParameter {
        name: "detail",
        description:
            "Rendered parser detail when a more specific syntax error message is available.",
    },
    RemediationParameter {
        name: "snippet",
        description: "Source snippet near the syntax failure when pato can recover it.",
    },
];
const P002_PARAMETERS: &[RemediationParameter] = &[
    RemediationParameter {
        name: "identity",
        description: "The duplicate identity string that must be made unique.",
    },
    RemediationParameter {
        name: "first_line",
        description: "Line number of the first definition site for the duplicate identity.",
    },
    RemediationParameter {
        name: "first_column",
        description: "Column number of the first definition site for the duplicate identity.",
    },
];
const P003_PARAMETERS: &[RemediationParameter] = &[RemediationParameter {
    name: "key",
    description: "The repeated annotation property key that should only appear once in the chain.",
}];
const P004_PARAMETERS: &[RemediationParameter] = &[
    RemediationParameter {
        name: "label_kind",
        description: "Whether the observed label is a node label or a relationship label.",
    },
    RemediationParameter {
        name: "observed",
        description: "The label spelling found in source before canonical recasing.",
    },
    RemediationParameter {
        name: "expected",
        description: "The canonical label spelling pato expects after recasing.",
    },
];
const P005_OPTIONS: &[RemediationOptionTemplate] = &[
    RemediationOptionTemplate {
        id: "rename-reference",
        summary: "Rename the unresolved reference to the closest known identity",
    },
    RemediationOptionTemplate {
        id: "add-definition",
        summary: "Add a new definition for the unresolved identity",
    },
];
const P005_PARAMETERS: &[RemediationParameter] = &[
    RemediationParameter {
        name: "unresolved_identity",
        description: "The reference that did not resolve to any definition in the file.",
    },
    RemediationParameter {
        name: "suggested_identity",
        description: "The closest in-file identity pato found as a rename candidate.",
    },
];
const P008_PARAMETERS: &[RemediationParameter] = &[RemediationParameter {
    name: "kind",
    description: "The unrecognized document header kind value that should be changed or removed.",
}];

const P001_REMEDIATION: RemediationTemplate = RemediationTemplate {
    id: "fix-gram-syntax",
    summary: "Fix the syntax near the reported location",
    details: "Correct the surrounding gram syntax and re-run `pato lint`.",
    parameters: P001_PARAMETERS,
    option_templates: NO_OPTIONS,
};

const P002_REMEDIATION: RemediationTemplate = RemediationTemplate {
    id: "rename-duplicate-identity",
    summary: "Rename one duplicate definition so each identity is unique",
    details: "Choose one of the duplicate definition sites and rename its identity.",
    parameters: P002_PARAMETERS,
    option_templates: NO_OPTIONS,
};

const P003_REMEDIATION: RemediationTemplate = RemediationTemplate {
    id: "remove-duplicate-annotation",
    summary: "Remove the duplicate annotation key from the annotation chain",
    details: "Keep only one annotation with the repeated key before the target pattern.",
    parameters: P003_PARAMETERS,
    option_templates: NO_OPTIONS,
};

const P004_REMEDIATION: RemediationTemplate = RemediationTemplate {
    id: "recase-label",
    summary: "Rewrite the label into the canonical case",
    details: "This remediation is deterministic and can be applied automatically.",
    parameters: P004_PARAMETERS,
    option_templates: NO_OPTIONS,
};

const P005_REMEDIATION: RemediationTemplate = RemediationTemplate {
    id: "resolve-dangling-reference",
    summary: "Resolve the reference by renaming it or by adding a matching definition",
    details: "Pick the option that matches the intended meaning of the source file.",
    parameters: P005_PARAMETERS,
    option_templates: P005_OPTIONS,
};

const P006_REMEDIATION: RemediationTemplate = RemediationTemplate {
    id: "replace-empty-array",
    summary: "Replace the empty array with a concrete value or remove the property",
    details: "Empty arrays are discouraged because they usually hide missing data or an omitted property.",
    parameters: NO_PARAMETERS,
    option_templates: NO_OPTIONS,
};

const P008_REMEDIATION: RemediationTemplate = RemediationTemplate {
    id: "fix-document-kind",
    summary: "Use a recognized pato document kind or remove the header property",
    details: "Recognized pato kinds in v0.1 are `diagnostics` and `rule`.",
    parameters: P008_PARAMETERS,
    option_templates: NO_OPTIONS,
};

const P001_REMEDIATIONS: &[RemediationTemplate] = &[P001_REMEDIATION];
const P002_REMEDIATIONS: &[RemediationTemplate] = &[P002_REMEDIATION];
const P003_REMEDIATIONS: &[RemediationTemplate] = &[P003_REMEDIATION];
const P004_REMEDIATIONS: &[RemediationTemplate] = &[P004_REMEDIATION];
const P005_REMEDIATIONS: &[RemediationTemplate] = &[P005_REMEDIATION];
const P006_REMEDIATIONS: &[RemediationTemplate] = &[P006_REMEDIATION];
const P007_REMEDIATIONS: &[RemediationTemplate] = &[];
const P008_REMEDIATIONS: &[RemediationTemplate] = &[P008_REMEDIATION];

const P001_RULE: RuleInfo = RuleInfo {
    code: DiagnosticCode::P001,
    name: "parse-failure",
    severity: Severity::Error,
    grade: RemediationGrade::Guided,
    description: "The file could not be parsed as gram.",
    remediations: P001_REMEDIATIONS,
    trigger_example_gram: "(",
};

const P002_RULE: RuleInfo = RuleInfo {
    code: DiagnosticCode::P002,
    name: "no-duplicate-identity",
    severity: Severity::Error,
    grade: RemediationGrade::Guided,
    description: "An identity is defined more than once in the same file.",
    remediations: P002_REMEDIATIONS,
    trigger_example_gram: "(alice:Person)\n(alice:Employee)",
};

const P003_RULE: RuleInfo = RuleInfo {
    code: DiagnosticCode::P003,
    name: "no-duplicate-annotation-key",
    severity: Severity::Error,
    grade: RemediationGrade::Guided,
    description: "The same annotation key appears more than once in a single annotation chain.",
    remediations: P003_REMEDIATIONS,
    trigger_example_gram: "@@meta:Doc @source(primary) @source(backup) (alice:Person)",
};

const P004_RULE: RuleInfo = RuleInfo {
    code: DiagnosticCode::P004,
    name: "label-case",
    severity: Severity::Warning,
    grade: RemediationGrade::Auto,
    description: "Labels should use the canonical case for their syntactic role.",
    remediations: P004_REMEDIATIONS,
    trigger_example_gram: "(alice:Person)-[:knows]->(bob:Person)",
};

const P005_RULE: RuleInfo = RuleInfo {
    code: DiagnosticCode::P005,
    name: "dangling-reference",
    severity: Severity::Warning,
    grade: RemediationGrade::Ambiguous,
    description: "A referenced identity does not resolve to a definition in the same file.",
    remediations: P005_REMEDIATIONS,
    trigger_example_gram: "(alice:Person)-->(bob)",
};

const P006_RULE: RuleInfo = RuleInfo {
    code: DiagnosticCode::P006,
    name: "empty-array",
    severity: Severity::Info,
    grade: RemediationGrade::Guided,
    description: "An empty array was used as a property value.",
    remediations: P006_REMEDIATIONS,
    trigger_example_gram: "(alice {tags: []})",
};

const P007_RULE: RuleInfo = RuleInfo {
    code: DiagnosticCode::P007,
    name: "no-schema",
    severity: Severity::Info,
    grade: RemediationGrade::None,
    description: "No matching schema file was found for the input.",
    remediations: P007_REMEDIATIONS,
    trigger_example_gram: "(alice:Person)",
};

const P008_RULE: RuleInfo = RuleInfo {
    code: DiagnosticCode::P008,
    name: "unknown-document-kind",
    severity: Severity::Warning,
    grade: RemediationGrade::Guided,
    description: "The document header uses an unrecognized `kind` value.",
    remediations: P008_REMEDIATIONS,
    trigger_example_gram: "{ kind: \"unknownkind\" }\n(alice:Person)",
};

const ALL_RULE_INFOS: &[RuleInfo] = &[
    P001_RULE, P002_RULE, P003_RULE, P004_RULE, P005_RULE, P006_RULE, P007_RULE, P008_RULE,
];
pub const ALL_DIAGNOSTIC_CODES: &[DiagnosticCode] = &[
    DiagnosticCode::P001,
    DiagnosticCode::P002,
    DiagnosticCode::P003,
    DiagnosticCode::P004,
    DiagnosticCode::P005,
    DiagnosticCode::P006,
    DiagnosticCode::P007,
    DiagnosticCode::P008,
];

pub fn rule_info(code: DiagnosticCode) -> &'static RuleInfo {
    ALL_RULE_INFOS
        .iter()
        .find(|info| info.code == code)
        .expect("every diagnostic code should have rule info")
}

pub fn all_rule_infos() -> &'static [RuleInfo] {
    ALL_RULE_INFOS
}

pub fn option_template(
    code: DiagnosticCode,
    option_id: &str,
) -> Option<&'static RemediationOptionTemplate> {
    let remediation = rule_info(code).remediations.first()?;
    remediation
        .option_templates
        .iter()
        .find(|template| template.id == option_id)
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
