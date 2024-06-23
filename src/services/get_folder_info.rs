use std::path::{Path, PathBuf};
use crate::services::scan_dir::scan_dir;

pub fn get_folder_info(folder_path: &Path) -> (usize, u64, PathBuf) {
    let mut total_files = 0;
    let mut total_size = 0;
    let mut folder_path_buf = PathBuf::new();

    scan_dir(folder_path, &mut |entry| {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                total_files += 1;
                total_size += metadata.len();
            }
        }
    });

    folder_path_buf.push(folder_path);

    (total_files, total_size, folder_path_buf)
}