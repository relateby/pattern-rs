use crate::skill_install::SkillInstallError;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/skill_bundle.rs"));

static CANONICAL_BUNDLE_ROOT: OnceLock<PathBuf> = OnceLock::new();

pub fn canonical_skill_root(project_root: &Path) -> PathBuf {
    project_root.join(".agents/skills/pato")
}

pub fn canonical_repository_skill_root() -> PathBuf {
    CANONICAL_BUNDLE_ROOT
        .get_or_init(|| {
            let root = std::env::temp_dir()
                .join(format!("relateby-pato-skill-bundle-{}", std::process::id()));

            if root.exists() {
                fs::remove_dir_all(&root).expect("stale skill bundle should be removable");
            }

            materialize_embedded_skill_bundle(&root)
                .expect("embedded canonical skill bundle should materialize");
            root
        })
        .clone()
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
