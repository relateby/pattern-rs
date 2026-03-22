use clap::{CommandFactory, FromArgMatches};
use relateby_pato::cli::{Cli, Commands};
use relateby_pato::commands::{check, fmt, help, lint, parse, rule, skill};
use relateby_pato::diagnostic_gram;
use relateby_pato::extensions;
use relateby_pato::output::{OutputContext, OutputFormat};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = parse_cli();

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
        Commands::Check(args) => {
            let output = OutputContext::new(OutputFormat::from(args.output_format));
            let outcome = check::check_paths(&args.files, args.schema.as_deref());
            if outcome.reports.is_empty() && outcome.had_io_error {
                return ExitCode::from(3);
            }
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
        Commands::Skill(args) => skill::run(args),
        Commands::Help(args) => help::run(&args),
        Commands::External(args) => {
            let Some((subcommand, forwarded)) = args.split_first() else {
                eprintln!("unknown subcommand: <unknown>");
                return ExitCode::from(3);
            };
            extensions::exec_extension(subcommand, forwarded)
        }
    }
}

fn parse_cli() -> Cli {
    let mut command = Cli::command();
    let extensions = extensions::discover_extensions();
    if !extensions.is_empty() {
        command = command.after_help(render_extensions_help(&extensions));
    }

    let matches = command.get_matches();
    Cli::from_arg_matches(&matches).unwrap_or_else(|error| error.exit())
}

fn render_extensions_help(extensions: &[(String, Option<String>)]) -> String {
    let mut lines = vec!["Extensions:".to_string()];
    for (name, description) in extensions {
        match description {
            Some(description) => lines.push(format!("  {name:<20} {description}")),
            None => lines.push(format!("  {name}")),
        }
    }
    lines.join("\n")
}
