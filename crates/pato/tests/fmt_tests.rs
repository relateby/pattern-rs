use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn fmt_transforms_fixture_pairs() {
    let cases = [
        (
            "valid/fmt_before_spacing.gram",
            "valid/fmt_after_spacing.gram",
        ),
        ("valid/fmt_before_props.gram", "valid/fmt_after_props.gram"),
        (
            "valid/fmt_before_header.gram",
            "valid/fmt_after_header.gram",
        ),
        (
            "valid/fmt_before_comments.gram",
            "valid/fmt_after_comments.gram",
        ),
    ];

    for (before, after) in cases {
        let temp_dir = unique_temp_dir();
        let target = temp_dir.join(Path::new(before).file_name().unwrap());
        fs::copy(fixture_path(before), &target).expect("fixture should copy");

        let output = run_pato(["fmt", target.to_str().expect("path should be utf8")]);
        assert_eq!(
            output.status.code(),
            Some(0),
            "unexpected exit code for {before}"
        );

        let actual = fs::read_to_string(&target).expect("formatted file should exist");
        let expected =
            fs::read_to_string(fixture_path(after)).expect("expected fixture should load");
        assert_eq!(actual, expected, "unexpected formatted output for {before}");
    }
}

#[test]
fn fmt_check_reports_needed_changes_and_clean_files() {
    let temp_dir = unique_temp_dir();
    let before = temp_dir.join("fmt_before_spacing.gram");
    let clean = temp_dir.join("fmt_after_spacing.gram");
    fs::copy(fixture_path("valid/fmt_before_spacing.gram"), &before).unwrap();
    fs::copy(fixture_path("valid/fmt_after_spacing.gram"), &clean).unwrap();

    let before_output = run_pato([
        "fmt",
        "--check",
        before.to_str().expect("path should be utf8"),
    ]);
    assert_eq!(before_output.status.code(), Some(1));
    let before_stderr = String::from_utf8(before_output.stderr).expect("stderr should be utf8");
    assert!(before_stderr.contains(before.to_str().expect("path should be utf8")));

    let clean_output = run_pato([
        "fmt",
        "--check",
        clean.to_str().expect("path should be utf8"),
    ]);
    assert_eq!(clean_output.status.code(), Some(0));
    let clean_stderr = String::from_utf8(clean_output.stderr).expect("stderr should be utf8");
    assert!(!clean_stderr.contains(clean.to_str().expect("path should be utf8")));
}

#[test]
fn fmt_stdin_writes_formatted_stdout() {
    let input = fs::read_to_string(fixture_path("valid/fmt_before_props.gram")).unwrap();
    let expected = fs::read_to_string(fixture_path("valid/fmt_after_props.gram")).unwrap();
    let output = run_pato_with_stdin(["fmt", "-"], &input);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
}

#[test]
fn fmt_rewrites_lowercase_relationship_label_and_lints_without_auto_fixes() {
    let temp_dir = unique_temp_dir();
    let target = temp_dir.join("P004.gram");
    fs::copy(fixture_path("invalid/P004.gram"), &target).unwrap();

    let fmt_output = run_pato(["fmt", target.to_str().expect("path should be utf8")]);
    assert_eq!(fmt_output.status.code(), Some(0));

    let contents = fs::read_to_string(&target).unwrap();
    assert!(contents.contains(":KNOWS"));

    let lint_output = run_pato(["lint", target.to_str().expect("path should be utf8")]);
    assert_eq!(lint_output.status.code(), Some(0));
    let lint_stdout = String::from_utf8(lint_output.stdout).unwrap();
    assert!(!lint_stdout.contains("code: \"P004\""));
}

#[test]
fn fmt_is_idempotent_for_valid_fixtures() {
    let valid_dir = fixture_path("valid");
    let entries = fs::read_dir(valid_dir).expect("valid fixtures should exist");

    for entry in entries {
        let entry = entry.expect("fixture entry should load");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("gram") {
            continue;
        }

        let temp_dir = unique_temp_dir();
        let target = temp_dir.join(path.file_name().expect("file name should exist"));
        fs::copy(&path, &target).expect("fixture should copy");

        let first = run_pato(["fmt", target.to_str().expect("path should be utf8")]);
        assert_eq!(
            first.status.code(),
            Some(0),
            "first fmt pass should succeed for {}",
            path.display()
        );
        let once = fs::read_to_string(&target).unwrap();

        let second = run_pato(["fmt", target.to_str().expect("path should be utf8")]);
        assert_eq!(
            second.status.code(),
            Some(0),
            "second fmt pass should succeed for {}",
            path.display()
        );
        let twice = fs::read_to_string(&target).unwrap();
        assert_eq!(
            once,
            twice,
            "fmt should be idempotent for {}",
            path.display()
        );
    }
}

#[test]
fn fmt_parse_errors_exit_two_without_writing_stdout() {
    let temp_dir = unique_temp_dir();
    let invalid = temp_dir.join("broken.gram");
    fs::write(&invalid, "@").unwrap();

    let output = run_pato(["fmt", invalid.to_str().expect("path should be utf8")]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
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

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("pato-fmt-tests-{nonce}"));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}
