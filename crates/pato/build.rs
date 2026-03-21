use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let source_root = canonical_skill_root(&manifest_dir);
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"));

    // Collect files under the canonical skill root
    let mut entries: Vec<(String, PathBuf)> = Vec::new();
    collect_files(&source_root, &source_root, &mut entries);
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

/// Determine the canonical skill root to embed. Preference order:
/// 1. workspace `.agents/skills/pato` (if present)
/// 2. `skill-package/pato` in the crate (if present)
/// 3. fallback to `skill-package/pato` (even if absent) so build errors are clear
fn canonical_skill_root(manifest_dir: &Path) -> PathBuf {
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

    // Fallback: return the packaged path so build errors later surface clearly
    packaged_root
}
