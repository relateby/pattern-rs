use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn default_project_install_creates_skill_and_reports_path() {
    let project_root = unique_temp_dir("pato-skill-project");
    let output = run_pato(&project_root, None, ["skill"]);

    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    let installed_path = project_root.join(".agents/skills/pato");
    assert!(stdout.contains(&installed_path.display().to_string()));
    assert!(installed_path.join("SKILL.md").exists());
    assert!(stdout.contains("installed pato skill"));
}

#[test]
fn print_path_only_outputs_resolved_path() {
    let project_root = unique_temp_dir("pato-skill-print-path");
    let output = run_pato(&project_root, None, ["skill", "--print-path"]);

    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    let installed_path = fs::canonicalize(project_root.join(".agents/skills/pato"))
        .expect("installed path should exist");
    assert_eq!(stdout.trim(), installed_path.display().to_string());
}

#[test]
fn supported_scope_and_target_combinations_install_to_expected_locations() {
    let project_root = unique_temp_dir("pato-skill-combos-project");
    let home_root = unique_temp_dir("pato-skill-combos-home");

    let user_interoperable = run_pato(
        &project_root,
        Some(&home_root),
        ["skill", "--scope", "user"],
    );
    assert_eq!(user_interoperable.status.code(), Some(0));
    assert!(home_root.join(".agents/skills/pato/SKILL.md").exists());

    let user_cursor = run_pato(
        &project_root,
        Some(&home_root),
        ["skill", "--scope", "user", "--target", "cursor", "--force"],
    );
    assert_eq!(user_cursor.status.code(), Some(0));
    assert!(home_root.join(".cursor/skills/pato/SKILL.md").exists());
}

#[test]
fn project_cursor_install_is_rejected() {
    let project_root = unique_temp_dir("pato-skill-project-cursor");
    let output = run_pato(
        &project_root,
        None,
        ["skill", "--scope", "project", "--target", "cursor"],
    );

    assert_eq!(output.status.code(), Some(3));

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("project installs must use"));
}

#[test]
fn repeated_install_requires_force_to_replace_existing_skill() {
    let project_root = unique_temp_dir("pato-skill-replace");

    let first = run_pato(&project_root, None, ["skill"]);
    assert_eq!(first.status.code(), Some(0));

    let second = run_pato(&project_root, None, ["skill"]);
    assert_eq!(second.status.code(), Some(3));
    let stderr = String::from_utf8(second.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("replacement was not requested"));

    let forced = run_pato(&project_root, None, ["skill", "--force"]);
    assert_eq!(forced.status.code(), Some(0));
}

#[test]
fn canonical_package_has_skill_metadata_and_support_files() {
    let repo_root = repo_root();
    let skill_root = repo_root.join(".agents/skills/pato");

    let contents = fs::read_to_string(skill_root.join("SKILL.md")).expect("SKILL.md should load");
    assert!(contents.contains("name: pato"));
    assert!(contents.contains("description:"));
    assert!(skill_root.join("references/workflows.md").exists());
    assert!(skill_root.join("references/output-contracts.md").exists());
    assert!(skill_root.join("assets/examples.md").exists());
}

#[test]
fn cargo_package_includes_the_canonical_skill_tree() {
    let repo_root = repo_root();
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let canonical_skill_root = repo_root.join(".agents/skills/pato");
    let packaged_skill_root = crate_root.join("skill-package/pato");

    let canonical_skill = fs::read_to_string(canonical_skill_root.join("SKILL.md"))
        .expect("canonical SKILL.md should load");
    let packaged_skill = fs::read_to_string(packaged_skill_root.join("SKILL.md"))
        .expect("packaged SKILL.md should load");
    assert_eq!(packaged_skill, canonical_skill);

    let output = Command::new("cargo")
        .current_dir(crate_root)
        .args(["package", "--allow-dirty", "--no-verify", "--list"])
        .output()
        .expect("cargo package should run");

    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("README.md"));
    assert!(stdout.contains("skill-package/pato/SKILL.md"));
    assert!(stdout.contains("skill-package/pato/references/workflows.md"));
    assert!(stdout.contains("skill-package/pato/references/output-contracts.md"));
    assert!(stdout.contains("skill-package/pato/assets/examples.md"));
}

fn run_pato<I, S>(cwd: &Path, home: Option<&Path>, args: I) -> std::process::Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut command = Command::new(env!("CARGO_BIN_EXE_pato"));
    command.current_dir(cwd).args(args);

    if let Some(home) = home {
        command.env("HOME", home);
        command.env("USERPROFILE", home);
    }

    command.output().expect("pato command should run")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root should exist")
        .to_path_buf()
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("{prefix}-{nonce}"));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}
