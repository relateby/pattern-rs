use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let source_root = canonical_skill_root(&manifest_dir);
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("out dir"));
    let bundle_root = out_dir.join("pato-skill-bundle");

    copy_tree(&source_root, &bundle_root).expect("canonical skill package should copy");
    println!(
        "cargo:rustc-env=PATO_SKILL_BUNDLE_DIR={}",
        bundle_root.display()
    );
    println!("cargo:rerun-if-changed={}", source_root.display());
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

fn copy_tree(source: &Path, destination: &Path) -> io::Result<()> {
    if destination.exists() {
        fs::remove_dir_all(destination)?;
    }
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_tree(&source_path, &destination_path)?;
        } else {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}
