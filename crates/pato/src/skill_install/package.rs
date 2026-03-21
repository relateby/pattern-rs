/// Filesystem-backed canonical skill package handlers.
///
/// This module avoids relying on build-script emitted `OUT_DIR` includes and
/// instead locates the canonical skill package on the repository/workspace
/// filesystem. It provides:
///
/// - `canonical_skill_root(project_root)` to resolve the canonical install
///   destination for a project (i.e. `<project_root>/.agents/skills/pato`).
/// - `validate_canonical_bundle()` which ensures the canonical package exists
///   and that its `SKILL.md` declares `name: pato` and includes a
///   `description:` field.
/// - `install_skill_from_bundle(destination)` which copies the canonical
///   package tree into `destination`.
///
/// The resolution strategy prefers an on-disk workspace `./.agents/skills/pato`
/// when present; otherwise it falls back to a packaged mirror under this crate:
/// `skill-package/pato`. When neither exists, functions return clear errors so
/// callers (and tests) can fail with actionable messages.
use crate::skill_install::SkillInstallError;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Returns the canonical skill path for a given project root:
/// `<project_root>/.agents/skills/pato`
pub fn canonical_skill_root(project_root: &Path) -> PathBuf {
    project_root.join(".agents/skills/pato")
}

/// Validate the canonical bundle found on disk. This will:
/// 1. Locate the repository/workspace canonical source of the skill package.
/// 2. Read `SKILL.md` and ensure it contains `name: pato` and a `description:`.
///
/// If no canonical package is present, or the SKILL.md is missing / malformed,
/// an appropriate `SkillInstallError` is returned.
pub fn validate_canonical_bundle() -> Result<(), SkillInstallError> {
    let bundle_root = match locate_canonical_bundle() {
        Ok(p) => p,
        Err((path, source)) => {
            return Err(SkillInstallError::CanonicalPackageMissing { path, source })
        }
    };

    let skill_md_path = bundle_root.join("SKILL.md");
    let contents = fs::read_to_string(&skill_md_path).map_err(|source| {
        SkillInstallError::CanonicalPackageMissing {
            path: skill_md_path.clone(),
            source,
        }
    })?;

    if !contents.contains("name: pato") {
        return Err(SkillInstallError::InvalidCanonicalPackage {
            path: skill_md_path,
            reason: "SKILL.md frontmatter must declare name: pato".to_string(),
        });
    }

    if !contents.contains("description:") {
        return Err(SkillInstallError::InvalidCanonicalPackage {
            path: skill_md_path,
            reason: "SKILL.md frontmatter must include a description".to_string(),
        });
    }

    Ok(())
}

/// Install the canonical bundle by copying its files into `destination`.
/// If `destination` already exists, this returns an `AlreadyExists` io::Error.
///
/// The copy is performed recursively, preserving relative paths.
pub fn install_skill_from_bundle(destination: &Path) -> io::Result<()> {
    if destination.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "destination already exists: {}; remove it before calling install_skill_from_bundle",
                destination.display()
            ),
        ));
    }

    let bundle_root = match locate_canonical_bundle() {
        Ok(b) => b,
        Err((path, source)) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "canonical skill package not found: {}; underlying: {}",
                    path.display(),
                    source
                ),
            ))
        }
    };

    copy_dir_all(&bundle_root, destination)
}

/// Helper: locate the canonical bundle on disk.
///
/// Preference order:
/// 1. Workspace `.agents/skills/pato` (workspace root = ancestors().nth(2))
/// 2. Packaged mirror `skill-package/pato` rooted at this crate's manifest dir
fn locate_canonical_bundle() -> Result<PathBuf, (PathBuf, io::Error)> {
    // Determine repository/workspace root by walking up from this crate's manifest.
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    // workspace root is two ancestors above the crate manifest in the workspace layout
    let workspace_root = manifest_dir
        .ancestors()
        .nth(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.to_path_buf());

    let workspace_skill = workspace_root.join(".agents/skills/pato");
    if workspace_skill.is_dir() {
        return Ok(workspace_skill);
    }

    // Fallback to the packaged mirror inside the crate
    let packaged = manifest_dir.join("skill-package/pato");
    if packaged.is_dir() {
        return Ok(packaged);
    }

    // Not found: return the workspace_skill path as the attempted location along with an io::Error
    Err((
        workspace_skill,
        io::Error::new(
            io::ErrorKind::NotFound,
            "no canonical skill bundle found in workspace or packaged mirror",
        ),
    ))
}

/// Recursively copy a directory. Source must be a directory. Creates `dst` and all
/// intermediate directories. Returns the first error encountered, if any.
fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("source is not a directory: {}", src.display()),
        ));
    }

    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&from, &to)?;
        } else if file_type.is_file() {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&from, &to)?;
        } else {
            // ignore symlinks and other file types for now
        }
    }
    Ok(())
}
