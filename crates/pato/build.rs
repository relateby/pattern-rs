use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let source_root = canonical_skill_root(&manifest_dir);
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("out dir"));

    let mut entries: Vec<(String, PathBuf)> = Vec::new();
    collect_files(&source_root, &source_root, &mut entries);
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut code = String::from("pub static SKILL_BUNDLE: &[(&str, &[u8])] = &[\n");
    for (rel_path, abs_path) in &entries {
        code.push_str(&format!(
            "    ({:?}, include_bytes!({:?})),\n",
            rel_path,
            abs_path.to_str().expect("skill path must be valid UTF-8"),
        ));
    }
    code.push_str("];\n");

    let bundle_path = out_dir.join("skill_bundle.rs");
    fs::write(&bundle_path, code).expect("should write skill_bundle.rs");

    emit_rerun_if_changed(&source_root);
}

fn collect_files(root: &Path, current: &Path, entries: &mut Vec<(String, PathBuf)>) {
    if let Ok(read_dir) = fs::read_dir(current) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(root, &path, entries);
            } else if path.is_file() {
                let rel = path.strip_prefix(root).expect("path must be inside root");
                let rel_str = rel
                    .to_str()
                    .expect("skill path must be valid UTF-8")
                    .replace('\\', "/");
                entries.push((rel_str, path));
            }
        }
    }
}

fn emit_rerun_if_changed(root: &Path) {
    println!("cargo:rerun-if-changed={}", root.display());
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                emit_rerun_if_changed(&path);
            } else {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}

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

    packaged_root
}
