use std::path::Path;
use crate::services::scan_dir::scan_dir;

pub fn list_files(folder_path: &Path) -> Vec<String> {
    let mut files = Vec::new();

    scan_dir(folder_path, &mut |entry| {
        if entry.path().is_file() {
            if let Some(file_name) = entry.file_name().to_str() {
                files.push(file_name.to_string());
            }
        }
    });

    files
}