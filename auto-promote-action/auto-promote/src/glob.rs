use anyhow::Result;
use glob::glob;
use std::path::PathBuf;

// Get all file paths matching the unix glob pattern.
pub fn files(pattern: &str) -> Result<Vec<PathBuf>> {
    let paths = glob(pattern)?
        .filter(|path| match path {
            Err(_) => true,
            Ok(path) => {
                if path.is_file() {
                    true
                } else {
                    false
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(paths)
}
