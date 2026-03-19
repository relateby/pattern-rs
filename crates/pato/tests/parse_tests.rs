use relateby_pato::commands::fmt::format_gram;
use serde_json::Value as JsonValue;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[test]
fn parse_gram_output_round_trips_stably() {
    for fixture in ["valid/simple.gram", "valid/commented.gram"] {
        let first = run_pato(["parse", fixture_path(fixture).to_str().unwrap()]);
        assert_eq!(
            first.status.code(),
            Some(0),
            "first parse failed for {fixture}"
        );
        let first_stdout = String::from_utf8(first.stdout).unwrap();
        let canonical = format_gram(&first_stdout).expect("parse gram output should be canonical");
        assert_eq!(
            first_stdout, canonical,
            "parse output should already be canonical"
        );

        let second = run_pato_with_stdin(["parse", "-"], &first_stdout);
        assert_eq!(
            second.status.code(),
            Some(0),
            "second parse failed for {fixture}"
        );
        let second_stdout = String::from_utf8(second.stdout).unwrap();
        assert_eq!(
            first_stdout, second_stdout,
            "round-trip changed output for {fixture}"
        );
    }
}

#[test]
fn parse_json_output_is_a_parseable_array() {
    let output = run_pato([
        "parse",
        "--output-format",
        "json",
        fixture_path("valid/simple.gram").to_str().unwrap(),
    ]);
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: JsonValue = serde_json::from_str(&stdout).expect("stdout should be valid json");
    let array = json
        .as_array()
        .expect("parse json output should be an array");
    assert!(!array.is_empty());
    assert_eq!(array[0]["subject"]["properties"]["kind"], "rule");
}

#[test]
fn parse_sexp_matches_corpus_relationship_fixture() {
    let input = "(hello)-->(world)\n";
    let expected = r#"(gram_pattern
  (relationship_pattern
      left: (node_pattern
        identifier: (symbol))
      kind: (right_arrow)
      right: (node_pattern
        identifier: (symbol))))"#;

    let output = run_pato_with_stdin(["parse", "--output-format", "sexp", "-"], input);
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(normalize_sexp(&stdout), normalize_sexp(expected));
}

#[test]
fn parse_sexp_matches_corpus_annotation_fixture() {
    let input = "@description(\"Two nodes in a relationship\")\n()-->()\n";
    let expected = r#"(gram_pattern
  (annotated_pattern
    annotations: (annotations
      (property_annotation
        key: (symbol)
        value: (string_literal
          content: (string_content))))
    elements: (relationship_pattern
      left: (node_pattern)
      kind: (right_arrow)
      right: (node_pattern))))"#;

    let output = run_pato_with_stdin(["parse", "--output-format", "sexp", "-"], input);
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(normalize_sexp(&stdout), normalize_sexp(expected));
}

#[test]
fn parse_summary_counts_annotations_and_walks_from_cst() {
    let input = "@description(\"path\")\n(a)-->(b)-->(c)\n";
    let output = run_pato_with_stdin(["parse", "--output-format", "summary", "-"], input);
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("nodes: 3"));
    assert!(stdout.contains("relationships: 2"));
    assert!(stdout.contains("annotations: 1"));
    assert!(stdout.contains("walks: 1"));
}

#[test]
fn parse_reports_parse_errors_on_stderr() {
    let output = run_pato(["parse", fixture_path("invalid/P001.gram").to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("failed to parse"));
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

fn run_pato_with_stdin<I, S>(args: I, stdin: &str) -> std::process::Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut child = Command::new(env!("CARGO_BIN_EXE_pato"))
        .args(args)
        .current_dir(repo_root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("pato command should spawn");

    child
        .stdin
        .as_mut()
        .expect("stdin should be available")
        .write_all(stdin.as_bytes())
        .expect("stdin write should succeed");

    child
        .wait_with_output()
        .expect("pato command should complete")
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

fn normalize_sexp(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n")
}
