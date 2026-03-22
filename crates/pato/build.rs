use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let source_root = authoritative_skill_root(&manifest_dir);
    let packaged_root = manifest_dir.join("skill-package/pato");
    if source_root != packaged_root {
        sync_tree(&source_root, &packaged_root);
    }
    let reference_root = packaged_root.join("reference");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"));

    // Collect topic files under the canonical reference root.
    let mut topics: Vec<(String, PathBuf)> = Vec::new();
    collect_markdown_files(&reference_root, &mut topics);
    if topics.is_empty() {
        panic!(
            "no topic markdown files found under {}",
            reference_root.display()
        );
    }
    topics.sort_by(|a, b| a.0.cmp(&b.0));

    let mut topic_catalog = String::from("pub static TOPICS: &[TopicEntry] = &[\n");
    for (name, abs_path) in &topics {
        let abs_path_str = abs_path.to_str().expect("topic path must be valid UTF-8");
        topic_catalog.push_str(&format!(
            "    TopicEntry {{ name: {:?}, content: include_str!({:?}) }},\n",
            name, abs_path_str
        ));
    }
    topic_catalog.push_str("];\n");
    fs::write(out_dir.join("topic_catalog.rs"), topic_catalog)
        .expect("failed to write topic_catalog.rs");

    // Collect files under the canonical skill root for the install bundle.
    let mut entries: Vec<(String, PathBuf)> = Vec::new();
    collect_files(&packaged_root, &packaged_root, &mut entries);
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    // Emit a Rust source file that statically embeds the bundle using include_bytes!
    let mut code = String::from("pub static SKILL_BUNDLE: &[(&str, &[u8])] = &[\n");
    for (rel_path, abs_path) in &entries {
        // Use debug formatting for the include_bytes! argument so the generated code has a valid
        // string literal (quotes and escapes included).
        let abs_path_str = abs_path.to_str().expect("skill path must be valid UTF-8");
        code.push_str(&format!(
            "    ({:?}, include_bytes!({:?})),\n",
            rel_path, abs_path_str
        ));
    }
    code.push_str("];\n");

    let bundle_path = out_dir.join("skill_bundle.rs");
    fs::write(&bundle_path, code).expect("failed to write skill_bundle.rs");

    // Tell cargo to re-run the build script if anything in the source tree changes
    emit_rerun_if_changed(&source_root);
    if source_root != packaged_root {
        emit_rerun_if_changed(&packaged_root);
    }
}

/// Collect markdown topic files directly under the reference directory.
fn collect_markdown_files(root: &Path, entries: &mut Vec<(String, PathBuf)>) {
    let read_dir = match fs::read_dir(root) {
        Ok(rd) => rd,
        Err(_) => return,
    };

    for entry in read_dir.flatten() {
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

        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(name) if !name.is_empty() => name.to_string(),
            _ => continue,
        };

        entries.push((stem, path));
    }
}

/// Recursively collect files under `current`, recording their path relative to `root`
/// and the absolute path to the file.
fn collect_files(root: &Path, current: &Path, entries: &mut Vec<(String, PathBuf)>) {
    let read_dir = match fs::read_dir(current) {
        Ok(rd) => rd,
        Err(_) => return,
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, entries);
        } else if path.is_file() {
            let rel = match path.strip_prefix(root) {
                Ok(r) => r,
                Err(_) => continue,
            };
            let rel_str = rel
                .to_str()
                .expect("skill path must be valid UTF-8")
                .replace('\\', "/");
            entries.push((rel_str, path));
        }
    }
}

/// Emit cargo:rerun-if-changed for a path and all children so the build script runs
/// when anything in the skill tree changes.
fn emit_rerun_if_changed(root: &Path) {
    println!("cargo:rerun-if-changed={}", root.display());

    if let Ok(mut entries) = fs::read_dir(root) {
        while let Some(Ok(entry)) = entries.next() {
            let path = entry.path();
            if path.is_dir() {
                emit_rerun_if_changed(&path);
            } else {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}

/// Determine the authoritative source skill root.
///
/// Preference order:
/// 1. workspace `.agents/skills/pato` (authoritative in this repo)
/// 2. packaged `skill-package/pato` tree (used by published source packages)
fn authoritative_skill_root(manifest_dir: &Path) -> PathBuf {
    let workspace_root = manifest_dir
        .ancestors()
        .nth(2)
        .expect("workspace root should exist")
        .join(".agents/skills/pato");
    if workspace_root.is_dir() {
        return workspace_root;
    }

    let packaged_root = manifest_dir.join("skill-package/pato");
    if packaged_root.is_dir() {
        return packaged_root;
    }

    packaged_root
}

/// Copy the authoritative tree into the packaged tree, replacing any stale files.
fn sync_tree(source_root: &Path, packaged_root: &Path) {
    if packaged_root.exists() {
        if let Ok(entries) = fs::read_dir(packaged_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(&path).expect("failed to remove stale packaged directory");
                } else {
                    fs::remove_file(&path).expect("failed to remove stale packaged file");
                }
            }
        }
    } else {
        fs::create_dir_all(packaged_root).expect("failed to create packaged skill root");
    }

    copy_tree(source_root, packaged_root);
}

/// Recursively copy a directory tree from `source_root` to `destination_root`.
fn copy_tree(source_root: &Path, destination_root: &Path) {
    let read_dir = fs::read_dir(source_root).expect("failed to read authoritative skill root");
    for entry in read_dir.flatten() {
        let source_path = entry.path();
        let destination_path = destination_root.join(entry.file_name());
        if source_path.is_dir() {
            fs::create_dir_all(&destination_path).expect("failed to create destination directory");
            copy_tree(&source_path, &destination_path);
        } else if source_path.is_file() {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent).expect("failed to create parent directory");
            }
            fs::copy(&source_path, &destination_path).expect("failed to copy packaged file");
        }
    }
}
