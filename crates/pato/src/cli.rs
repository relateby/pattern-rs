use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "pato", version, about = "CLI tooling for gram files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormatArg {
    Gram,
    Text,
    Json,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ParseOutputFormatArg {
    Gram,
    Sexp,
    Json,
    Summary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum RuleOutputFormatArg {
    Gram,
    Json,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum SkillScopeArg {
    Project,
    User,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum SkillTargetArg {
    Interoperable,
    Cursor,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Lint(LintArgs),
    Fmt(FmtArgs),
    Parse(ParseArgs),
    Rule(RuleArgs),
    Check(CheckArgs),
    Skill(SkillArgs),
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Debug, clap::Args)]
pub struct LintArgs {
    #[arg(long)]
    pub fix: bool,

    #[arg(long, value_enum, default_value_t = OutputFormatArg::Gram)]
    pub output_format: OutputFormatArg,

    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct FmtArgs {
    #[arg(long)]
    pub check: bool,

    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct ParseArgs {
    #[arg(long, value_enum, default_value_t = ParseOutputFormatArg::Gram)]
    pub output_format: ParseOutputFormatArg,

    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct RuleArgs {
    #[arg(long, value_enum, default_value_t = RuleOutputFormatArg::Gram)]
    pub output_format: RuleOutputFormatArg,

    pub code: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct CheckArgs {
    #[arg(long)]
    pub schema: Option<PathBuf>,

    #[arg(long, value_enum, default_value_t = OutputFormatArg::Gram)]
    pub output_format: OutputFormatArg,

    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct SkillArgs {
    #[arg(long, value_enum, default_value_t = SkillScopeArg::Project)]
    pub scope: SkillScopeArg,

    #[arg(long, value_enum, default_value_t = SkillTargetArg::Interoperable)]
    pub target: SkillTargetArg,

    #[arg(long)]
    pub force: bool,

    #[arg(long)]
    pub print_path: bool,
}
