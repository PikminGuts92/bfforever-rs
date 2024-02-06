use std::io::Result as IOResult;
use std::fs::create_dir_all;
use std::path::Path;

pub fn create_missing_dirs<T: AsRef<Path>>(file_path: T) -> IOResult<()> {
    let file_path = file_path.as_ref();

    let dir = if file_path.is_dir() {
        Some(file_path)
    } else {
        file_path.parent()
    };

    // Create directory
    if let Some(output_dir) = dir {
        if !output_dir.exists() {
            create_dir_all(&output_dir)?;
        }
    }

    Ok(())
}