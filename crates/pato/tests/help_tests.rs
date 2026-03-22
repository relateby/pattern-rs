use relateby_pato::topic_catalog::TOPICS;
use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn known_topic_prints_content_to_stdout() {
    let output = run_pato(["help", "gram"]);

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("# Gram"));
    assert!(stdout.contains("```gram"));
    assert!(stdout.contains("```"));
}

#[test]
fn unknown_topic_reports_error_and_lists_topics_on_stderr() {
    let output = run_pato(["help", "no-such-topic"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("error: unknown topic: no-such-topic"));
    assert!(stderr.contains("available topics:"));
    for topic in TOPICS {
        assert!(
            stderr.contains(topic.name),
            "stderr should list {}",
            topic.name
        );
    }
}

#[test]
fn missing_topic_reports_usage_and_lists_topics_on_stderr() {
    let output = run_pato(["help"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("usage: pato help <topic>"));
    assert!(stderr.contains("available topics:"));
    for topic in TOPICS {
        assert!(
            stderr.contains(topic.name),
            "stderr should list {}",
            topic.name
        );
    }
}

#[test]
fn all_catalog_entries_resolve_to_non_empty_content() {
    assert!(!TOPICS.is_empty());
    for topic in TOPICS {
        assert!(!topic.name.is_empty());
        assert!(
            !topic.content.trim().is_empty(),
            "topic {} should not be empty",
            topic.name
        );
    }
}

#[test]
fn catalog_matches_reference_markdown_files() {
    let mut file_names = Vec::new();
    let reference_root = repo_root().join(".agents/skills/pato/reference");
    for entry in fs::read_dir(&reference_root).expect("reference dir should be readable") {
        let entry = entry.expect("reference entry should be readable");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let is_markdown = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "md")
            .unwrap_or(false);
        if !is_markdown {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("topic file should have a valid stem");
        file_names.push(stem.to_string());
    }
    file_names.sort();

    let mut catalog_names: Vec<_> = TOPICS.iter().map(|topic| topic.name).collect();
    catalog_names.sort();

    assert_eq!(catalog_names, file_names);
}

fn run_pato<I, S>(args: I) -> std::process::Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    Command::new(env!("CARGO_BIN_EXE_pato"))
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("pato command should run")
}

fn repo_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root should exist")
        .to_path_buf()
}
