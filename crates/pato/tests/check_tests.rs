use gram_codec::parse_gram;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn check_without_schema_reports_p007_and_preserves_lint_exit_status() {
    let fixture = fixture_path("invalid/P005.gram");
    let output = run_pato([
        "check",
        fixture.to_str().expect("fixture path should be utf8"),
    ]);
    assert_eq!(output.status.code(), Some(1));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("code: \"P005\""));
    assert!(stdout.contains("code: \"P007\""));
    parse_gram(&stdout).expect("check output should be valid gram");
}

#[test]
fn check_with_same_stem_schema_suppresses_p007_and_logs_schema_path() {
    let temp_dir = unique_temp_dir();
    let data_path = temp_dir.join("sample.gram");
    let schema_path = temp_dir.join("sample.schema.gram");

    fs::copy(fixture_path("valid/sample.gram"), &data_path).expect("data fixture should copy");
    fs::copy(fixture_path("schema/sample.schema.gram"), &schema_path)
        .expect("schema fixture should copy");

    let output = run_pato(["check", data_path.to_str().expect("path should be utf8")]);
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(!stdout.contains("code: \"P007\""));
    assert!(stdout.contains("Summary"));
    parse_gram(&stdout).expect("check output should be valid gram");

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains(&format!("using schema: {}", schema_path.display())));
}

#[test]
fn check_with_schema_override_acknowledges_specified_schema() {
    let data_path = fixture_path("valid/simple.gram");
    let schema_path = fixture_path("schema/sample.schema.gram");

    let output = run_pato([
        "check",
        "--schema",
        schema_path.to_str().expect("schema path should be utf8"),
        data_path.to_str().expect("data path should be utf8"),
    ]);
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(!stdout.contains("code: \"P007\""));
    assert!(stdout.contains("Summary"));
    parse_gram(&stdout).expect("check output should be valid gram");

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains(&format!("using schema: {}", schema_path.display())));
}

#[test]
fn check_with_missing_schema_override_exits_three() {
    let missing_schema = fixture_path("schema/missing.schema.gram");
    let data_path = fixture_path("valid/simple.gram");

    let output = run_pato([
        "check",
        "--schema",
        missing_schema.to_str().expect("schema path should be utf8"),
        data_path.to_str().expect("data path should be utf8"),
    ]);
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("failed to access schema"));
    assert!(stderr.contains(&missing_schema.display().to_string()));
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
    let path = std::env::temp_dir().join(format!("pato-check-tests-{nonce}"));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}
