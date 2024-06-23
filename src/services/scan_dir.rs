use std::fs::DirEntry;
use std::path::Path;

pub fn scan_dir(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) {
    if let Ok(entries) = dir.read_dir() {
        for entry in entries.flatten() {
            cb(&entry);

            if entry.path().is_dir() {
                scan_dir(&entry.path(), cb);
            }
        }
    }
}