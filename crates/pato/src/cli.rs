use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "pato",
    version,
    disable_help_subcommand = true,
    about = "CLI tooling for gram files",
    long_about = "CLI tooling for gram files.\n\nExamples:\n  pato --help\n  pato skill\n  pato help gram"
)]
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
    #[command(
        about = "Lint gram files",
        long_about = "Lint gram files and optionally apply fixes."
    )]
    Lint(LintArgs),
    #[command(
        about = "Format gram files",
        long_about = "Format gram files in place or check formatting without writing changes."
    )]
    Fmt(FmtArgs),
    #[command(
        about = "Parse gram files",
        long_about = "Parse gram files and render the result in a chosen output format."
    )]
    Parse(ParseArgs),
    #[command(
        about = "Render grammar rules",
        long_about = "Render grammar rules from gram code or from the current input stream."
    )]
    Rule(RuleArgs),
    #[command(
        about = "Check gram files",
        long_about = "Check gram files against an optional schema and report diagnostics."
    )]
    Check(CheckArgs),
    #[command(
        about = "Manage the installed skill tree",
        long_about = "Install or update the pato skill tree for the current scope."
    )]
    Skill(SkillArgs),
    #[command(
        about = "Show help for a topic",
        long_about = "Show the embedded help topic for a given subject."
    )]
    Help(HelpArgs),
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

#[derive(Debug, clap::Args)]
/// Help topic arguments.
pub struct HelpArgs {
    /// The topic name to show, such as `gram`.
    pub topic: Option<String>,
}
