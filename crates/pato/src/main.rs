use clap::Parser;
use relateby_pato::cli::{Cli, Commands};
use relateby_pato::commands::{fmt, lint, parse, rule};
use relateby_pato::diagnostic_gram;
use relateby_pato::output::{OutputContext, OutputFormat};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lint(args) => {
            let output = OutputContext::new(OutputFormat::from(args.output_format));
            let outcome = lint::lint_paths(&args.files, args.fix);
            if let Err(error) = diagnostic_gram::render_diagnostics(
                &outcome.reports,
                output.format,
                &mut std::io::stdout(),
                output.use_color,
            ) {
                eprintln!("failed to render diagnostics: {error}");
                return ExitCode::from(3);
            }
            ExitCode::from(outcome.exit_code() as u8)
        }
        Commands::Fmt(args) => {
            let outcome = fmt::format_paths(&args.files, args.check);
            if let Some(stdout) = outcome.stdout {
                print!("{stdout}");
            }
            ExitCode::from(outcome.exit_code as u8)
        }
        Commands::Parse(args) => {
            let outcome = parse::parse_paths(&args.files, args.output_format.into());
            if let Some(stdout) = outcome.stdout {
                print!("{stdout}");
            }
            ExitCode::from(outcome.exit_code as u8)
        }
        Commands::Rule(args) => {
            let outcome = rule::render_rules(args.code.as_deref(), args.output_format.into());
            if let Some(stdout) = outcome.stdout {
                print!("{stdout}");
            }
            ExitCode::from(outcome.exit_code as u8)
        }
        Commands::Check(_) => {
            eprintln!("not yet implemented");
            ExitCode::SUCCESS
        }
        Commands::External(args) => {
            let command = args.first().map(String::as_str).unwrap_or("<unknown>");
            eprintln!("unknown subcommand: {command}");
            ExitCode::from(3)
        }
    }
}
