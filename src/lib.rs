use std::path::PathBuf;

use walkdir::WalkDir;

#[derive(Debug, serde::Serialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug, serde::Serialize)]
pub struct DirectoryReport {
    pub total_size: u64,
    pub top_files: Vec<FileInfo>,
}

pub fn scan_dir(path: &PathBuf, top_n: usize) -> DirectoryReport {
    let mut files = Vec::new();
    let mut total_size = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                let size = metadata.len();
                total_size += size;
                files.push(FileInfo {
                    path: entry.path().to_path_buf(),
                    size,
                });
            }
        }
    }

    files.sort_by_key(|f| std::cmp::Reverse(f.size));
    let top_files = files.into_iter().take(top_n).collect();

    DirectoryReport {
        total_size,
        top_files,
    }
}
