use crate::diagnostics::Edit;
use std::fs;
use std::io;
use std::path::Path;

pub fn apply_edits(file: &Path, edits: &[Edit]) -> io::Result<()> {
    let original = fs::read_to_string(file)?;
    let mut lines: Vec<String> = original.lines().map(ToString::to_string).collect();
    let had_trailing_newline = original.ends_with('\n');

    let mut ordered = edits.to_vec();
    ordered.sort_by_key(edit_sort_key);
    ordered.reverse();

    for edit in ordered {
        match edit {
            Edit::Replace {
                line,
                column,
                replace,
                with,
                ..
            } => {
                let line_index = usize::try_from(line.saturating_sub(1))
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid line"))?;
                let column_index = usize::try_from(column.saturating_sub(1))
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid column"))?;
                let current = lines.get_mut(line_index).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "replace line out of bounds")
                })?;
                let suffix = current.get(column_index..).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "replace column out of bounds")
                })?;
                if !suffix.starts_with(&replace) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "replacement text not found at column",
                    ));
                }
                let start = column_index;
                let end = start + replace.len();
                current.replace_range(start..end, &with);
            }
            Edit::DeleteLine { line, .. } => {
                let line_index = usize::try_from(line.saturating_sub(1))
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid line"))?;
                if line_index >= lines.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "delete line out of bounds",
                    ));
                }
                lines.remove(line_index);
            }
            Edit::Append { content, .. } => {
                lines.extend(content.lines().map(ToString::to_string));
            }
        }
    }

    let mut rewritten = lines.join("\n");
    if had_trailing_newline || !rewritten.is_empty() {
        rewritten.push('\n');
    }

    let temp_path = file.with_extension(format!(
        "{}.pato.tmp",
        file.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("tmp")
    ));
    fs::write(&temp_path, rewritten)?;
    fs::rename(temp_path, file)?;
    eprintln!("modified {}", file.display());
    Ok(())
}

fn edit_sort_key(edit: &Edit) -> (u32, u32) {
    match edit {
        Edit::Replace { line, column, .. } => (*line, *column),
        Edit::DeleteLine { line, .. } => (*line, u32::MAX),
        Edit::Append { .. } => (u32::MAX, u32::MAX),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn replace_requires_exact_column_match() {
        let file = temp_file_path("replace_requires_exact_column_match");
        fs::write(&file, "beta alpha\n").unwrap();

        let result = apply_edits(
            &file,
            &[Edit::Replace {
                file: PathBuf::from(&file),
                line: 1,
                column: 1,
                replace: "alpha".to_string(),
                with: "gamma".to_string(),
            }],
        );

        assert!(result.is_err());
        assert_eq!(fs::read_to_string(&file).unwrap(), "beta alpha\n");
        let _ = fs::remove_file(file);
    }

    fn temp_file_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("pattern-rs-{name}-{nanos}.gram"))
    }
}
