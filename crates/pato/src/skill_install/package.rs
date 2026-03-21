use crate::skill_install::SkillInstallError;
use std::fs;
use std::path::{Path, PathBuf};

pub fn canonical_skill_root(project_root: &Path) -> PathBuf {
    project_root.join(".agents/skills/pato")
}

pub fn canonical_repository_skill_root() -> PathBuf {
    PathBuf::from(env!("PATO_SKILL_BUNDLE_DIR"))
}

pub fn validate_canonical_package(root: &Path) -> Result<(), SkillInstallError> {
    let skill_md = root.join("SKILL.md");
    let contents = fs::read_to_string(&skill_md).map_err(|error| {
        SkillInstallError::CanonicalPackageMissing {
            path: skill_md.clone(),
            source: error,
        }
    })?;

    if !contents.contains("name: pato") {
        return Err(SkillInstallError::InvalidCanonicalPackage {
            path: skill_md,
            reason: "SKILL.md frontmatter must declare name: pato".to_string(),
        });
    }

    if !contents.contains("description:") {
        return Err(SkillInstallError::InvalidCanonicalPackage {
            path: skill_md,
            reason: "SKILL.md frontmatter must include a description".to_string(),
        });
    }

    if !root.is_dir() {
        return Err(SkillInstallError::CanonicalPackageMissing {
            path: root.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "canonical package root missing",
            ),
        });
    }

    Ok(())
}
