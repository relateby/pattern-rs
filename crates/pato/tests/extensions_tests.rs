use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn unknown_extension_exits_three_with_stderr_message() {
    let temp_dir = unique_temp_dir();
    let output = run_pato_with_path(["unknown-subcommand"], &temp_dir);

    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("unknown subcommand"));
    assert!(stderr.contains("unknown-subcommand"));
}

#[test]
fn external_subcommand_executes_and_forwards_args() {
    let temp_dir = unique_temp_dir();
    create_extension(&temp_dir, "foo", "Echo test extension");

    let output = run_pato_with_path(["foo", "--test", "value"], &temp_dir);
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("extension invoked"));
    assert!(stdout.contains("--test value"));
}

#[test]
fn help_lists_discovered_extensions_with_descriptions() {
    let temp_dir = unique_temp_dir();
    create_extension(&temp_dir, "foo", "Echo test extension");

    let output = run_pato_with_path(["--help"], &temp_dir);
    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("Extensions"));
    assert!(stdout.contains("foo"));
    assert!(stdout.contains("Echo test extension"));
}

fn run_pato_with_path<I, S>(args: I, path_dir: &Path) -> std::process::Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    Command::new(env!("CARGO_BIN_EXE_pato"))
        .args(args)
        .current_dir(repo_root())
        .env("PATH", prefixed_path(path_dir))
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

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("pato-extension-tests-{nonce}"));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

fn prefixed_path(path_dir: &Path) -> OsString {
    let mut paths = vec![path_dir.to_path_buf()];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    std::env::join_paths(paths).expect("path entries should join")
}

#[cfg(unix)]
fn create_extension(dir: &Path, name: &str, description: &str) {
    use std::os::unix::fs::PermissionsExt;

    let path = dir.join(format!("pato-{name}"));
    let script = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--pato-describe\" ]; then\n  printf '%s\\n' \"{description}\"\n  exit 0\nfi\nprintf 'extension invoked: %s\\n' \"$*\"\n"
    );
    fs::write(&path, script).expect("script should be written");

    let mut permissions = fs::metadata(&path)
        .expect("script metadata should exist")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).expect("script should be executable");
}

#[cfg(windows)]
fn create_extension(dir: &Path, name: &str, description: &str) {
    let path = dir.join(format!("pato-{name}.cmd"));
    let script = format!(
        "@echo off\r\nif \"%1\"==\"--pato-describe\" (\r\n  echo {description}\r\n  exit /b 0\r\n)\r\necho extension invoked: %*\r\n"
    );
    fs::write(&path, script).expect("script should be written");
}
