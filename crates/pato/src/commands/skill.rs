use crate::cli::SkillArgs;
use crate::skill_install::{
    install_skill_with_context, InstallRequest, InstallResult, SkillInstallError,
};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn run(args: SkillArgs) -> ExitCode {
    let project_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let home_dir = match crate::skill_install::target::home_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(error.exit_code());
        }
    };

    match execute_with_context(&args, &project_root, &home_dir) {
        Ok(outcome) => {
            println!("{}", outcome.rendered);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(error.exit_code())
        }
    }
}

#[derive(Debug)]
pub struct SkillCommandOutcome {
    pub result: InstallResult,
    pub rendered: String,
}

pub fn execute_with_context(
    args: &SkillArgs,
    project_root: &Path,
    home_dir: &Path,
) -> Result<SkillCommandOutcome, SkillInstallError> {
    let request = InstallRequest {
        project_root: project_root.to_path_buf(),
        scope: args.scope,
        target: args.target,
        allow_replace: args.force,
    };
    let result = install_skill_with_context(&request, project_root, home_dir)?;
    let rendered = if args.print_path {
        result.installed_path.display().to_string()
    } else {
        format!(
            "installed pato skill to {}",
            result.installed_path.display()
        )
    };

    Ok(SkillCommandOutcome { result, rendered })
}
