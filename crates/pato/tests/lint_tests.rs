use gram_codec::parse_gram;
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
