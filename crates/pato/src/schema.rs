use std::path::{Path, PathBuf};

pub fn discover_schema(data_file: &Path, override_path: Option<&Path>) -> Option<PathBuf> {
    if let Some(path) = override_path {
        return Some(path.to_path_buf());
    }

    let parent = data_file.parent()?;
    let stem = data_file.file_stem()?.to_str()?;
    let candidate = parent.join(format!("{stem}.schema.gram"));
    candidate.exists().then_some(candidate)
}
