use crate::skill_install::SkillInstallError;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Returns the canonical skill path for a given project root:
/// `<project_root>/.agents/skills/pato`
pub fn canonical_skill_root(project_root: &Path) -> PathBuf {
    project_root.join(".agents/skills/pato")
}

/// Validate the embedded bundle. This ensures the binary contains the expected
/// skill metadata before installation proceeds.
pub fn validate_canonical_bundle() -> Result<(), SkillInstallError> {
    let skill_md = crate::SKILL_BUNDLE
        .iter()
        .find(|(path, _)| *path == "SKILL.md")
        .map(|(_, content)| *content)
        .ok_or_else(|| SkillInstallError::EmbeddedBundleMissing {
            path: PathBuf::from("SKILL.md"),
        })?;

    let contents = std::str::from_utf8(skill_md).map_err(|source| {
        SkillInstallError::EmbeddedBundleInvalidUtf8 {
            path: PathBuf::from("SKILL.md"),
            source,
        }
    })?;

    if !contents.contains("name: pato") {
        return Err(SkillInstallError::InvalidEmbeddedBundle {
            path: PathBuf::from("SKILL.md"),
            reason: "SKILL.md frontmatter must declare name: pato".to_string(),
        });
    }

    if !contents.contains("description:") {
        return Err(SkillInstallError::InvalidEmbeddedBundle {
            path: PathBuf::from("SKILL.md"),
            reason: "SKILL.md frontmatter must include a description".to_string(),
        });
    }

    Ok(())
}

/// Install the embedded bundle by writing its files into `destination`.
/// If `destination` already exists, this returns an `AlreadyExists` io::Error.
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

    fs::create_dir_all(destination)?;
    for &(relative_path, content) in crate::SKILL_BUNDLE {
        let to = destination.join(relative_path);
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&to, content)?;
    }
    Ok(())
}
