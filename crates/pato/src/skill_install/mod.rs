pub mod package;
pub mod target;

pub use crate::cli::{SkillScopeArg, SkillTargetArg};
use crate::skill_install::package::{install_skill_from_bundle, validate_canonical_bundle};
use crate::skill_install::target::{home_dir, resolve_install_target};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstallStatus {
    Created,
    Replaced,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstallRequest {
    pub project_root: PathBuf,
    pub scope: SkillScopeArg,
    pub target: SkillTargetArg,
    pub allow_replace: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstallResult {
    pub status: InstallStatus,
    pub skill_name: String,
    pub installed_path: PathBuf,
    pub replaced_existing: bool,
    pub vercel_discoverable: bool,
}

#[derive(Debug, Error)]
pub enum SkillInstallError {
    #[error("unsupported install combination: {scope:?} + {target:?}: {reason}")]
    UnsupportedInstallCombination {
        scope: SkillScopeArg,
        target: SkillTargetArg,
        reason: String,
    },
    #[error("embedded skill bundle missing required file at {path}")]
    EmbeddedBundleMissing { path: PathBuf },
    #[error("embedded skill bundle contained invalid UTF-8 at {path}: {source}")]
    EmbeddedBundleInvalidUtf8 {
        path: PathBuf,
        source: std::str::Utf8Error,
    },
    #[error("embedded skill bundle invalid at {path}: {reason}")]
    InvalidEmbeddedBundle { path: PathBuf, reason: String },
    #[error("home directory unavailable")]
    HomeDirectoryUnavailable,
    #[error("install target already exists at {path}; re-run with --force to replace it")]
    ExistingInstallPresent { path: PathBuf },
    #[error("failed to write install target {path}: {source}")]
    InstallWriteFailed { path: PathBuf, source: io::Error },
    #[error("failed to remove existing install {path}: {source}")]
    ExistingInstallRemoveFailed { path: PathBuf, source: io::Error },
    #[error("failed to inspect install target {path}: {source}")]
    InstallInspectFailed { path: PathBuf, source: io::Error },
}

impl SkillInstallError {
    pub fn exit_code(&self) -> u8 {
        match self {
            SkillInstallError::ExistingInstallPresent { .. }
            | SkillInstallError::UnsupportedInstallCombination { .. }
            | SkillInstallError::HomeDirectoryUnavailable => 3,
            SkillInstallError::EmbeddedBundleMissing { .. }
            | SkillInstallError::EmbeddedBundleInvalidUtf8 { .. }
            | SkillInstallError::InvalidEmbeddedBundle { .. }
            | SkillInstallError::InstallWriteFailed { .. }
            | SkillInstallError::ExistingInstallRemoveFailed { .. }
            | SkillInstallError::InstallInspectFailed { .. } => 3,
        }
    }
}

pub fn install_skill(request: &InstallRequest) -> Result<InstallResult, SkillInstallError> {
    let home_dir = home_dir()?;
    install_skill_with_context(request, &request.project_root, &home_dir)
}

pub fn install_skill_with_context(
    request: &InstallRequest,
    project_root: &Path,
    home_dir: &Path,
) -> Result<InstallResult, SkillInstallError> {
    validate_canonical_bundle()?;

    let target = resolve_install_target(project_root, home_dir, request.scope, request.target)?;

    let had_existing = target.resolved_path.exists();

    if had_existing {
        if !request.allow_replace {
            return Err(SkillInstallError::ExistingInstallPresent {
                path: target.resolved_path,
            });
        }
        fs::remove_dir_all(&target.resolved_path).map_err(|source| {
            SkillInstallError::ExistingInstallRemoveFailed {
                path: target.resolved_path.clone(),
                source,
            }
        })?;
    }

    install_skill_from_bundle(&target.resolved_path).map_err(|source| {
        SkillInstallError::InstallWriteFailed {
            path: target.resolved_path.clone(),
            source,
        }
    })?;

    Ok(InstallResult {
        status: if had_existing {
            InstallStatus::Replaced
        } else {
            InstallStatus::Created
        },
        skill_name: "pato".to_string(),
        installed_path: target.resolved_path,
        replaced_existing: had_existing,
        vercel_discoverable: target.vercel_discoverable,
    })
}
