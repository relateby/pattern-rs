use crate::cli::{SkillScopeArg, SkillTargetArg};
use crate::skill_install::SkillInstallError;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstallTarget {
    pub scope: SkillScopeArg,
    pub target: SkillTargetArg,
    pub resolved_path: PathBuf,
    pub vercel_discoverable: bool,
}

pub fn resolve_install_target(
    project_root: &Path,
    home_dir: &Path,
    scope: SkillScopeArg,
    target: SkillTargetArg,
) -> Result<InstallTarget, SkillInstallError> {
    let resolved_path = match (scope, target) {
        (SkillScopeArg::Project, SkillTargetArg::Interoperable) => {
            project_root.join(".agents/skills/pato")
        }
        (SkillScopeArg::Project, SkillTargetArg::Cursor) => {
            return Err(SkillInstallError::UnsupportedInstallCombination {
                scope,
                target,
                reason: "project installs must use the interoperable `.agents/skills/` path"
                    .to_string(),
            });
        }
        (SkillScopeArg::User, SkillTargetArg::Interoperable) => {
            home_dir.join(".agents/skills/pato")
        }
        (SkillScopeArg::User, SkillTargetArg::Cursor) => home_dir.join(".cursor/skills/pato"),
    };

    Ok(InstallTarget {
        scope,
        target,
        vercel_discoverable: matches!(scope, SkillScopeArg::Project)
            && matches!(target, SkillTargetArg::Interoperable),
        resolved_path,
    })
}

pub fn home_dir() -> Result<PathBuf, SkillInstallError> {
    #[cfg(windows)]
    {
        if let Some(home) = env::var_os("USERPROFILE") {
            return Ok(PathBuf::from(home));
        }
        if let (Some(drive), Some(path)) = (env::var_os("HOMEDRIVE"), env::var_os("HOMEPATH")) {
            let mut home = std::ffi::OsString::from(drive);
            home.push(path);
            return Ok(PathBuf::from(home));
        }
    }

    #[cfg(not(windows))]
    {
        if let Some(home) = env::var_os("HOME") {
            return Ok(PathBuf::from(home));
        }
    }

    Err(SkillInstallError::HomeDirectoryUnavailable)
}
