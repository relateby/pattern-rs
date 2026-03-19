use crate::cli::RuleOutputFormatArg;
use crate::commands::fmt;
use crate::diagnostics::{
    all_rule_infos, DiagnosticCode, RemediationOptionTemplate, RemediationParameter,
    RemediationTemplate, RuleInfo,
};
use gram_codec::to_gram_with_header;
use pattern_core::{Pattern, Subject, Value};
use serde_json::json;
use std::collections::HashMap;

const PATO_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleOutputFormat {
    Gram,
    Json,
}

impl From<RuleOutputFormatArg> for RuleOutputFormat {
    fn from(value: RuleOutputFormatArg) -> Self {
        match value {
            RuleOutputFormatArg::Gram => RuleOutputFormat::Gram,
            RuleOutputFormatArg::Json => RuleOutputFormat::Json,
        }
    }
}

pub struct RuleOutcome {
    pub exit_code: i32,
    pub stdout: Option<String>,
}

pub fn render_rules(code: Option<&str>, output_format: RuleOutputFormat) -> RuleOutcome {
    let rules = match select_rules(code) {
        Ok(rules) => rules,
        Err(message) => {
            eprintln!("{message}");
            return RuleOutcome {
                exit_code: 3,
                stdout: None,
            };
        }
    };

    let stdout = match output_format {
        RuleOutputFormat::Gram => render_gram(&rules)
            .map_err(|error| error.to_string())
            .map(Some),
        RuleOutputFormat::Json => Ok(Some(render_json(&rules))),
    };

    match stdout {
        Ok(stdout) => RuleOutcome {
            exit_code: 0,
            stdout,
        },
        Err(error) => {
            eprintln!("failed to render rule catalog: {error}");
            RuleOutcome {
                exit_code: 3,
                stdout: None,
            }
        }
    }
}

fn select_rules(code: Option<&str>) -> Result<Vec<&'static RuleInfo>, String> {
    if let Some(code) = code {
        let Some(parsed) = DiagnosticCode::parse(code) else {
            return Err(format!("unknown diagnostic code: {code}"));
        };
        Ok(vec![crate::diagnostics::rule_info(parsed)])
    } else {
        Ok(all_rule_infos().iter().collect())
    }
}

fn render_gram(rules: &[&RuleInfo]) -> Result<String, gram_codec::SerializeError> {
    let raw = to_gram_with_header(header_record(), &rules_to_patterns(rules))?;
    Ok(fmt::format_gram(&raw).expect("generated rule gram should be canonical"))
}

fn render_json(rules: &[&RuleInfo]) -> String {
    let value = json!({
        "kind": "rule",
        "patoVersion": PATO_VERSION,
        "rules": rules.iter().map(|rule| rule_to_json(rule)).collect::<Vec<_>>()
    });
    let mut output = serde_json::to_string_pretty(&value).expect("rule json should serialize");
    output.push('\n');
    output
}

fn header_record() -> HashMap<String, Value> {
    HashMap::from([
        ("kind".to_string(), Value::VString("rule".to_string())),
        (
            "pato_version".to_string(),
            Value::VString(PATO_VERSION.to_string()),
        ),
    ])
}

fn rules_to_patterns(rules: &[&RuleInfo]) -> Vec<Pattern<Subject>> {
    rules
        .iter()
        .enumerate()
        .map(|(index, rule)| rule_pattern(index + 1, rule))
        .collect()
}

fn rule_pattern(index: usize, rule: &RuleInfo) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert(
        "code".to_string(),
        Value::VString(rule.code.as_str().to_string()),
    );
    properties.insert("name".to_string(), Value::VString(rule.name.to_string()));
    properties.insert(
        "severity".to_string(),
        Value::VString(rule.severity.as_str().to_string()),
    );
    properties.insert(
        "grade".to_string(),
        Value::VString(rule.grade.as_str().to_string()),
    );
    properties.insert(
        "description".to_string(),
        Value::VString(rule.description.to_string()),
    );

    let mut children = Vec::new();
    for (remediation_index, remediation) in rule.remediations.iter().enumerate() {
        children.push(remediation_pattern(
            index,
            remediation_index + 1,
            remediation,
        ));
        for (parameter_index, parameter) in remediation.parameters.iter().enumerate() {
            children.push(parameter_pattern(
                index,
                remediation_index + 1,
                parameter_index + 1,
                remediation.id,
                parameter,
            ));
        }
        for (option_index, option) in remediation.option_templates.iter().enumerate() {
            children.push(option_template_pattern(
                index,
                remediation_index + 1,
                option_index + 1,
                remediation.id,
                option,
            ));
        }
    }
    children.push(trigger_example_pattern(index, rule.trigger_example_gram));

    Pattern::pattern(
        subject(&format!("rule{index}"), &["Rule"], properties),
        children,
    )
}

fn remediation_pattern(
    rule_index: usize,
    remediation_index: usize,
    remediation: &RemediationTemplate,
) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert("id".to_string(), Value::VString(remediation.id.to_string()));
    properties.insert(
        "summary".to_string(),
        Value::VString(remediation.summary.to_string()),
    );
    properties.insert(
        "details".to_string(),
        Value::VString(remediation.details.to_string()),
    );

    Pattern::point(subject(
        &format!("remediation{rule_index}_{remediation_index}"),
        &["Remediation"],
        properties,
    ))
}

fn parameter_pattern(
    rule_index: usize,
    remediation_index: usize,
    parameter_index: usize,
    remediation_id: &str,
    parameter: &RemediationParameter,
) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert(
        "remediation".to_string(),
        Value::VString(remediation_id.to_string()),
    );
    properties.insert(
        "name".to_string(),
        Value::VString(parameter.name.to_string()),
    );
    properties.insert(
        "description".to_string(),
        Value::VString(parameter.description.to_string()),
    );
    Pattern::point(subject(
        &format!("parameter{rule_index}_{remediation_index}_{parameter_index}"),
        &["Parameter"],
        properties,
    ))
}

fn option_template_pattern(
    rule_index: usize,
    remediation_index: usize,
    option_index: usize,
    remediation_id: &str,
    option: &RemediationOptionTemplate,
) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert(
        "remediation".to_string(),
        Value::VString(remediation_id.to_string()),
    );
    properties.insert("id".to_string(), Value::VString(option.id.to_string()));
    properties.insert(
        "summary".to_string(),
        Value::VString(option.summary.to_string()),
    );
    Pattern::point(subject(
        &format!("option_template{rule_index}_{remediation_index}_{option_index}"),
        &["OptionTemplate"],
        properties,
    ))
}

fn trigger_example_pattern(rule_index: usize, trigger_example_gram: &str) -> Pattern<Subject> {
    let mut properties = HashMap::new();
    properties.insert(
        "gram".to_string(),
        Value::VString(trigger_example_gram.to_string()),
    );
    Pattern::point(subject(
        &format!("trigger{rule_index}"),
        &["TriggerExample"],
        properties,
    ))
}

fn rule_to_json(rule: &RuleInfo) -> serde_json::Value {
    json!({
        "code": rule.code.as_str(),
        "name": rule.name,
        "severity": rule.severity.as_str(),
        "grade": rule.grade.as_str(),
        "description": rule.description,
        "triggerExampleGram": rule.trigger_example_gram,
        "remediations": rule.remediations.iter().map(remediation_to_json).collect::<Vec<_>>()
    })
}

fn remediation_to_json(remediation: &RemediationTemplate) -> serde_json::Value {
    json!({
        "id": remediation.id,
        "summary": remediation.summary,
        "details": remediation.details,
        "parameters": remediation.parameters.iter().map(parameter_to_json).collect::<Vec<_>>(),
        "optionTemplates": remediation.option_templates.iter().map(option_template_to_json).collect::<Vec<_>>()
    })
}

fn parameter_to_json(parameter: &RemediationParameter) -> serde_json::Value {
    json!({
        "name": parameter.name,
        "description": parameter.description
    })
}

fn option_template_to_json(option: &RemediationOptionTemplate) -> serde_json::Value {
    json!({
        "id": option.id,
        "summary": option.summary
    })
}

fn subject(identity: &str, labels: &[&str], properties: HashMap<String, Value>) -> Subject {
    Subject {
        identity: pattern_core::Symbol(identity.to_string()),
        labels: labels.iter().map(|label| (*label).to_string()).collect(),
        properties,
    }
}
