use std::collections::BTreeSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const DESCRIBE_TIMEOUT: Duration = Duration::from_secs(1);
const POLL_INTERVAL: Duration = Duration::from_millis(10);

pub fn discover_extensions() -> Vec<(String, Option<String>)> {
    let mut seen = BTreeSet::new();
    let mut extensions = Vec::new();

    for directory in path_entries() {
        let Ok(entries) = std::fs::read_dir(directory) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let Some(binary_name) = discoverable_binary_name(&path) else {
                continue;
            };
            if !seen.insert(binary_name.clone()) {
                continue;
            }

            let description = query_description(&path);
            extensions.push((binary_name, description));
        }
    }

    extensions
}

pub fn exec_extension(subcommand: &str, args: &[String]) -> ! {
    let Some(binary) = resolve_extension_binary(subcommand) else {
        eprintln!("unknown subcommand: {subcommand}");
        std::process::exit(3);
    };

    let status = Command::new(&binary)
        .args(args)
        .status()
        .unwrap_or_else(|error| {
            eprintln!("failed to execute {}: {error}", binary.display());
            std::process::exit(3);
        });

    std::process::exit(status.code().unwrap_or(3));
}

fn resolve_extension_binary(subcommand: &str) -> Option<PathBuf> {
    let binary_name = format!("pato-{subcommand}");
    path_entries()
        .into_iter()
        .find_map(|directory| resolve_in_directory(&directory, &binary_name))
}

fn resolve_in_directory(directory: &Path, binary_name: &str) -> Option<PathBuf> {
    candidate_names(binary_name)
        .into_iter()
        .map(|candidate| directory.join(candidate))
        .find(|path| is_launchable_file(path))
}

fn discoverable_binary_name(path: &Path) -> Option<String> {
    if !is_launchable_file(path) {
        return None;
    }

    let file_name = path.file_name()?.to_str()?;
    #[cfg(windows)]
    {
        let stem = path.file_stem()?.to_str()?;
        if stem.starts_with("pato-") && has_windows_executable_extension(path) {
            return Some(stem.to_string());
        }
    }

    #[cfg(not(windows))]
    {
        if file_name.starts_with("pato-") {
            return Some(file_name.to_string());
        }
    }

    None
}

fn query_description(path: &Path) -> Option<String> {
    let mut child = Command::new(path)
        .arg("--pato-describe")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let output = child.wait_with_output().ok()?;
                if !status.success() {
                    return None;
                }
                let stdout = String::from_utf8(output.stdout).ok()?;
                let description = stdout.lines().next()?.trim();
                return (!description.is_empty()).then(|| description.to_string());
            }
            Ok(None) if started.elapsed() < DESCRIBE_TIMEOUT => {
                thread::sleep(POLL_INTERVAL);
            }
            Ok(None) | Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
}

fn path_entries() -> Vec<PathBuf> {
    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).collect())
        .unwrap_or_default()
}

fn candidate_names(binary_name: &str) -> Vec<String> {
    #[cfg(windows)]
    {
        use std::ffi::OsString;

        let mut names = Vec::new();
        let mut seen = BTreeSet::new();
        seen.insert(binary_name.to_string());
        names.push(binary_name.to_string());

        let pathext =
            env::var_os("PATHEXT").unwrap_or_else(|| OsString::from(".COM;.EXE;.BAT;.CMD"));
        for ext in pathext.to_string_lossy().split(';') {
            if ext.is_empty() {
                continue;
            }
            let candidate = format!("{binary_name}{}", ext.to_ascii_lowercase());
            if seen.insert(candidate.clone()) {
                names.push(candidate);
            }
            let upper = format!("{binary_name}{}", ext.to_ascii_uppercase());
            if seen.insert(upper.clone()) {
                names.push(upper);
            }
        }
        names
    }

    #[cfg(not(windows))]
    {
        vec![binary_name.to_string()]
    }
}

fn is_launchable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let Ok(metadata) = std::fs::metadata(path) else {
            return false;
        };
        metadata.permissions().mode() & 0o111 != 0
    }

    #[cfg(windows)]
    {
        has_windows_executable_extension(path)
    }
}

#[cfg(windows)]
fn has_windows_executable_extension(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
        return false;
    };
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "exe" | "cmd" | "bat" | "com"
    )
}
