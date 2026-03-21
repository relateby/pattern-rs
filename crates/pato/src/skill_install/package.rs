use crate::skill_install::SkillInstallError;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/skill_bundle.rs"));

pub fn canonical_skill_root(project_root: &Path) -> PathBuf {
    project_root.join(".agents/skills/pato")
}

pub fn validate_canonical_bundle() -> Result<(), SkillInstallError> {
    let skill_md_bytes = SKILL_BUNDLE
        .iter()
        .find(|(path, _)| *path == "SKILL.md")
        .map(|(_, bytes)| *bytes);

    let contents = match skill_md_bytes {
        None => {
            return Err(SkillInstallError::InvalidCanonicalPackage {
                path: PathBuf::from("SKILL.md"),
                reason: "SKILL.md not found in embedded bundle".to_string(),
            })
        }
        Some(bytes) => std::str::from_utf8(bytes).unwrap_or(""),
    };

    if !contents.contains("name: pato") {
        return Err(SkillInstallError::InvalidCanonicalPackage {
            path: PathBuf::from("SKILL.md"),
            reason: "SKILL.md frontmatter must declare name: pato".to_string(),
        });
    }

    if !contents.contains("description:") {
        return Err(SkillInstallError::InvalidCanonicalPackage {
            path: PathBuf::from("SKILL.md"),
            reason: "SKILL.md frontmatter must include a description".to_string(),
        });
    }

    Ok(())
}

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

    for (relative_path, bytes) in SKILL_BUNDLE {
        let full_path = destination.join(relative_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full_path, bytes)?;
    }

    Ok(())
}
