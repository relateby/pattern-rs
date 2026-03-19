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
                let relative = suffix.find(&replace).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "replacement text not found")
                })?;
                let start = column_index + relative;
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
