use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let source_root = canonical_skill_root(&manifest_dir);
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("out dir"));
    let bundle_source = out_dir.join("skill_bundle.rs");

    let mut files = Vec::new();
    collect_skill_files(&source_root, &source_root, &mut files)
        .expect("canonical skill package should be readable");
    files.sort_by(|left, right| left.0.cmp(&right.0));
    write_bundle_source(&bundle_source, &files).expect("canonical skill package should embed");
    emit_rerun_if_changed(&source_root).expect("rerun metadata should be writable");
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

fn collect_skill_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<(PathBuf, String)>,
) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_skill_files(root, &path, files)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .expect("source file should be under skill root")
                .to_path_buf();
            let contents = fs::read_to_string(&path)?;
            files.push((relative, contents));
        }
    }

    Ok(())
}

fn write_bundle_source(path: &Path, files: &[(PathBuf, String)]) -> io::Result<()> {
    let mut output = File::create(path)?;
    writeln!(
        output,
        "pub fn materialize_embedded_skill_bundle(root: &std::path::Path) -> std::io::Result<()> {{"
    )?;
    writeln!(output, "    use std::fs;")?;
    writeln!(output, "    fs::create_dir_all(root)?;")?;
    writeln!(output, "    for (relative_path, contents) in [")?;

    for (relative_path, contents) in files {
        writeln!(
            output,
            "        ({:?}, {:?}),",
            relative_path.to_string_lossy().replace('\\', "/"),
            contents
        )?;
    }

    writeln!(output, "    ] {{")?;
    writeln!(output, "        let path = root.join(relative_path);")?;
    writeln!(output, "        if let Some(parent) = path.parent() {{")?;
    writeln!(output, "            fs::create_dir_all(parent)?;")?;
    writeln!(output, "        }}")?;
    writeln!(output, "        fs::write(path, contents)?;")?;
    writeln!(output, "    }}")?;
    writeln!(output, "    Ok(())")?;
    writeln!(output, "}}")?;
    Ok(())
}

fn emit_rerun_if_changed(path: &Path) -> io::Result<()> {
    println!("cargo:rerun-if-changed={}", path.display());

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            emit_rerun_if_changed(&entry.path())?;
        }
    }

    Ok(())
}
