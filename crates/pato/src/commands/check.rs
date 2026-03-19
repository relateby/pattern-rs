use crate::commands::lint::lint_source;
use crate::diagnostics::{
    exit_code_for_reports, Diagnostic, DiagnosticCode, FileDiagnostics, Location, Remediation,
};
use crate::schema;
use std::fs;
use std::path::{Path, PathBuf};

pub struct CheckOutcome {
    pub reports: Vec<FileDiagnostics>,
    pub had_io_error: bool,
}

impl CheckOutcome {
    pub fn exit_code(&self) -> i32 {
        if self.had_io_error {
            3
        } else {
            exit_code_for_reports(&self.reports)
        }
    }
}

pub fn check_paths(files: &[PathBuf], override_schema: Option<&Path>) -> CheckOutcome {
    let mut reports = Vec::new();
    let mut had_io_error = false;

    for file in files {
        if file == Path::new("-") {
            had_io_error = true;
            eprintln!("stdin is not supported for `pato check`");
            continue;
        }

        let source = match fs::read_to_string(file) {
            Ok(source) => source,
            Err(error) => {
                had_io_error = true;
                eprintln!("failed to read {}: {error}", file.display());
                continue;
            }
        };

        let mut report = lint_source(&file.display().to_string(), &source, Some(file), false);
        match schema::discover_schema(file, override_schema) {
            Some(schema_path) => {
                if let Err(error) = validate_schema_path(&schema_path) {
                    had_io_error = true;
                    eprintln!("failed to access schema {}: {error}", schema_path.display());
                    continue;
                }
                eprintln!("using schema: {}", schema_path.display());
            }
            None => {
                report.diagnostics.push(no_schema_diagnostic());
                report.diagnostics.sort_by_key(|diagnostic| {
                    (diagnostic.location.line, diagnostic.location.column)
                });
            }
        }

        reports.push(report);
    }

    CheckOutcome {
        reports,
        had_io_error,
    }
}

fn validate_schema_path(path: &Path) -> std::io::Result<()> {
    let metadata = fs::metadata(path)?;
    if metadata.is_file() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "path is not a file",
        ))
    }
}

fn no_schema_diagnostic() -> Diagnostic {
    Diagnostic::new(DiagnosticCode::P007, Location::new(1, 1), Remediation::None)
}
