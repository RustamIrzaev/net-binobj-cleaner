use std::fs;
use std::path::PathBuf;

pub fn delete_folder(folder_path: &PathBuf) {
    if folder_path.exists() {
        if let Err(err) = fs::remove_dir_all(folder_path) {
            eprintln!("Failed to delete {:?}: {}", folder_path, err);
        }
    }
}