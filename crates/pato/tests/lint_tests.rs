use gram_codec::parse_gram;
use relateby_pato::commands::fmt::format_gram;
use relateby_pato::commands::lint::lint_source;
use relateby_pato::diagnostics::{rule_info, DiagnosticCode};
use serde_json::Value as JsonValue;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn lint_reports_expected_codes_and_exit_statuses() {
    let cases = [
        ("invalid/P001.gram", "P001", 2),
        ("invalid/P002.gram", "P002", 2),
        ("invalid/P003.gram", "P003", 2),
        ("invalid/P004.gram", "P004", 1),
        ("invalid/P005.gram", "P005", 1),
        ("invalid/P006.gram", "P006", 0),
        ("invalid/P008.gram", "P008", 1),
        ("valid/simple.gram", "Summary", 0),
        ("valid/commented.gram", "Summary", 0),
    ];

    for (fixture, expected, exit_code) in cases {
        let fixture_path = fixture_path(fixture);
        let output = run_pato([
            "lint",
            fixture_path.to_str().expect("fixture path should be utf8"),
        ]);
        assert_eq!(
            output.status.code(),
            Some(exit_code),
            "unexpected exit code for {fixture}"
        );

        let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
        assert!(
            stdout.contains(expected),
            "stdout for {fixture} should mention {expected}: {stdout}"
        );
        parse_gram(&stdout).expect("lint output should be valid gram");
    }
}

#[test]
fn lint_fix_rewrites_lowercase_relationship_label() {
    let temp_dir = unique_temp_dir();
    let source = fixture_path("invalid/P004.gram");
    let rewritten = temp_dir.join("P004.gram");
    fs::copy(source, &rewritten).expect("fixture should copy");

    let output = run_pato([
        "lint",
        "--fix",
        rewritten.to_str().expect("path should be utf8"),
    ]);
    assert_eq!(output.status.code(), Some(0));

    let contents = fs::read_to_string(&rewritten).expect("fixed file should exist");
    assert!(contents.contains(":KNOWS"));

    let second_pass = run_pato(["lint", rewritten.to_str().expect("path should be utf8")]);
    assert_eq!(second_pass.status.code(), Some(0));
    let stdout = String::from_utf8(second_pass.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("Summary"));
    parse_gram(&stdout).expect("post-fix lint output should be parseable");
}

#[test]
fn lint_uses_cst_locations_for_duplicate_identities() {
    let source =
        fs::read_to_string(fixture_path("invalid/P002.gram")).expect("fixture should load");
    let report = lint_source("P002.gram", &source, None, false);
    let duplicate = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == DiagnosticCode::P002)
        .expect("expected duplicate identity diagnostic");

    assert_eq!(duplicate.location.line, 2);
    assert_eq!(duplicate.location.column, 2);
    assert!(
        duplicate.message().contains("1:2"),
        "expected duplicate message to reference first CST-derived location: {}",
        duplicate.message()
    );
    assert_eq!(
        duplicate.remediation.id(),
        Some("rename-duplicate-identity")
    );
    assert_eq!(duplicate.rule, rule_info(duplicate.code).name);
}

#[test]
fn lint_handles_identified_annotations_when_reporting_duplicate_keys() {
    let source = fs::read_to_string(fixture_path("invalid/P003_identified.gram"))
        .expect("fixture should load");
    let report = lint_source("P003_identified.gram", &source, None, false);
    let duplicate = report
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == DiagnosticCode::P003)
        .expect("expected duplicate annotation diagnostic");

    let second_source = source
        .match_indices("@source")
        .nth(1)
        .map(|(offset, _)| offset)
        .expect("expected second property annotation");

    assert_eq!(duplicate.location.line, 1);
    assert_eq!(duplicate.location.column, second_source as u32 + 2);
}

#[test]
fn lint_gram_output_uses_problem_shape_and_registry_ids() {
    let fixture_path = fixture_path("invalid/P005.gram");
    let output = run_pato([
        "lint",
        fixture_path.to_str().expect("fixture path should be utf8"),
    ]);
    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("code: \"P005\""));
    assert!(stdout.contains("rule: \"dangling-reference\""));
    assert!(stdout.contains("remediation: \"resolve-dangling-reference\""));
    assert!(stdout.contains("id: \"rename-reference\""));
    assert!(stdout.contains("// "));
    assert!(!stdout.contains("message:"));
    assert!(!stdout.contains("decision:"));
    parse_gram(&stdout).expect("lint gram output should parse");
}

#[test]
fn lint_json_output_uses_compact_problem_shape() {
    let fixture_path = fixture_path("invalid/P005.gram");
    let output = run_pato([
        "lint",
        "--output-format",
        "json",
        fixture_path.to_str().expect("fixture path should be utf8"),
    ]);
    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    let json: JsonValue = serde_json::from_str(&stdout).expect("stdout should be valid json");
    let problem = &json["problems"][0];
    assert_eq!(problem["code"], "P005");
    assert_eq!(problem["rule"], "dangling-reference");
    assert_eq!(problem["remediation"], "resolve-dangling-reference");
    assert_eq!(problem["facts"]["unresolved_identity"], "persn");
    assert_eq!(problem["options"][0]["id"], "rename-reference");
    assert!(problem.get("message").is_none());
    assert!(problem.get("decision").is_none());
}

#[test]
fn lint_gram_output_is_already_canonical() {
    let fixture_path = fixture_path("invalid/P005.gram");
    let output = run_pato([
        "lint",
        fixture_path.to_str().expect("fixture path should be utf8"),
    ]);
    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    let canonical = format_gram(&stdout).expect("lint gram output should be formattable");
    assert_eq!(stdout, canonical);
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

fn fixture_path(relative: &str) -> PathBuf {
    repo_root()
        .join("crates/pato/tests/fixtures")
        .join(relative)
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root should exist")
        .to_path_buf()
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("pato-tests-{nonce}"));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}
