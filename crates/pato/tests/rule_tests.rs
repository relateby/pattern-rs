use gram_codec::parse_gram;
use relateby_pato::commands::lint::lint_source;
use relateby_pato::diagnostics::{all_rule_infos, DiagnosticCode};
use serde_json::Value as JsonValue;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn rule_registry_contains_all_codes_and_templates() {
    let infos = all_rule_infos();
    assert_eq!(infos.len(), 8);

    for info in infos {
        assert_eq!(info.code.rule_name(), info.name);
        assert_eq!(info.code.severity(), info.severity);
        assert_eq!(info.code.grade(), info.grade);
        assert!(!info.description.is_empty());
        assert!(!info.trigger_example_gram.is_empty());

        if let Some(remediation) = info.remediations.first() {
            assert_eq!(info.code.remediation_id(), Some(remediation.id));
        } else {
            assert_eq!(info.code, DiagnosticCode::P007);
            assert_eq!(info.code.remediation_id(), None);
        }
    }
}

#[test]
fn rule_listing_gram_output_parses_and_lists_all_rules() {
    let output = run_pato(["rule"]);
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("kind: \"rule\""));
    assert!(stdout.contains("Triggerexample"));
    assert!(stdout.contains("Remediation"));
    assert!(stdout.contains("Optiontemplate"));
    parse_gram(&stdout).expect("rule listing should be valid gram");

    for info in all_rule_infos() {
        assert!(stdout.contains(info.code.as_str()));
        assert!(stdout.contains(info.name));
        if let Some(remediation) = info.remediations.first() {
            assert!(stdout.contains(remediation.id));
        }
    }
}

#[test]
fn rule_detail_output_includes_trigger_example_and_remediation_detail() {
    let output = run_pato(["rule", "P002"]);
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("code: \"P002\""));
    assert!(stdout.contains("name: \"no-duplicate-identity\""));
    assert!(stdout.contains("grade: \"guided\""));
    assert!(stdout.contains("rename-duplicate-identity"));
    assert!(stdout.contains("gram: \"(alice:Person)\\n(alice:Employee)\""));
    parse_gram(&stdout).expect("rule detail should be valid gram");
}

#[test]
fn rule_json_output_lists_rules() {
    let output = run_pato(["rule", "--output-format", "json"]);
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    let json: JsonValue = serde_json::from_str(&stdout).expect("stdout should be valid json");
    assert_eq!(json["kind"], "rule");
    let rules = json["rules"].as_array().expect("rules should be an array");
    assert_eq!(rules.len(), all_rule_infos().len());
    assert_eq!(rules[0]["code"], "P001");
}

#[test]
fn rule_unknown_code_exits_three() {
    let output = run_pato(["rule", "P999"]);
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("unknown diagnostic code"));
}

#[test]
fn rule_trigger_examples_produce_their_claimed_lint_codes() {
    for info in all_rule_infos() {
        if info.code == DiagnosticCode::P007 {
            continue;
        }

        let report = lint_source(
            &format!("{}.gram", info.code.as_str()),
            info.trigger_example_gram,
            Some(Path::new("trigger.gram")),
            false,
        );
        let matching = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == info.code)
            .count();

        assert_eq!(
            matching,
            1,
            "trigger example for {} should produce exactly one matching diagnostic",
            info.code.as_str()
        );
    }
}

fn run_pato<I, S>(args: I) -> std::process::Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    Command::new(env!("CARGO_BIN_EXE_pato"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("pato command should run")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root should exist")
        .to_path_buf()
}
