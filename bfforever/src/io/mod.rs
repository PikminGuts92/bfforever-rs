use std::fs::{create_dir_all, File, remove_file};
use std::io::Error as IOError;
use std::path::Path;

pub fn create_new_file<T: AsRef<Path>>(file_path: T) -> Result<File, IOError> {
    let file_path = file_path.as_ref();

    /*if !file_path.is_file() {
        // TODO: Throw error?
    }*/

    // Create directory
    if let Some(output_dir) = file_path.parent() {
        if !output_dir.exists() {
            create_dir_all(&output_dir)?;
        }
    }

    // Delete old file
    if file_path.exists() {
        // TODO: Investigate better approach to guarantee deletion
        remove_file(file_path)?;
    }

    File::create(file_path)
}