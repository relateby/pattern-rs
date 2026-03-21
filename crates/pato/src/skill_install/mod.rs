pub mod package;
pub mod target;

pub use crate::cli::{SkillScopeArg, SkillTargetArg};
use crate::skill_install::package::{canonical_repository_skill_root, validate_canonical_package};
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
    pub source_root: PathBuf,
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
    #[error("canonical skill package missing at {path}: {source}")]
    CanonicalPackageMissing { path: PathBuf, source: io::Error },
    #[error("canonical skill package invalid at {path}: {reason}")]
    InvalidCanonicalPackage { path: PathBuf, reason: String },
    #[error("home directory unavailable")]
    HomeDirectoryUnavailable,
    #[error("install target already exists at {path} and replacement was not requested")]
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
            SkillInstallError::CanonicalPackageMissing { .. }
            | SkillInstallError::InvalidCanonicalPackage { .. }
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
    let source_root = canonical_repository_skill_root();
    validate_canonical_package(&source_root)?;

    let target = resolve_install_target(project_root, home_dir, request.scope, request.target)?;

    if same_path(&source_root, &target.resolved_path)? {
        return Ok(InstallResult {
            status: InstallStatus::Created,
            skill_name: "pato".to_string(),
            installed_path: target.resolved_path,
            replaced_existing: false,
            source_root,
            vercel_discoverable: target.vercel_discoverable,
        });
    }

    let replaced_existing = target.resolved_path.exists();

    if replaced_existing {
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

    copy_skill_tree(&source_root, &target.resolved_path).map_err(|source| {
        SkillInstallError::InstallWriteFailed {
            path: target.resolved_path.clone(),
            source,
        }
    })?;

    Ok(InstallResult {
        status: if replaced_existing {
            InstallStatus::Replaced
        } else {
            InstallStatus::Created
        },
        skill_name: "pato".to_string(),
        installed_path: target.resolved_path,
        replaced_existing,
        source_root,
        vercel_discoverable: target.vercel_discoverable,
    })
}

fn copy_skill_tree(source: &Path, destination: &Path) -> io::Result<()> {
    if destination.exists() {
        return Ok(());
    }

    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_skill_tree(&source_path, &destination_path)?;
        } else if source_path.is_file() {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}

fn same_path(left: &Path, right: &Path) -> Result<bool, SkillInstallError> {
    if !left.exists() || !right.exists() {
        return Ok(false);
    }

    let left = left
        .canonicalize()
        .map_err(|source| SkillInstallError::InstallInspectFailed {
            path: left.to_path_buf(),
            source,
        })?;
    let right = right
        .canonicalize()
        .map_err(|source| SkillInstallError::InstallInspectFailed {
            path: right.to_path_buf(),
            source,
        })?;
    Ok(left == right)
}
