use std::path::PathBuf;

use clap::Parser;
use humansize::{DECIMAL, format_size};
use walkdir::WalkDir;

#[derive(Parser)]
struct Args {
    // Path to the directory
    path: PathBuf,
    // Top heaviest files to show
    #[arg(long, default_value = "10")]
    top: usize,
}

#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    size: u64,
}

#[derive(Debug)]
struct DirectoryReport {
    total_size: u64,
    top_files: Vec<FileInfo>,
}

fn scan_dir(path: &PathBuf, top_n: usize) -> DirectoryReport {
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

fn main() {
    let args = Args::parse();
    let report = scan_dir(&args.path, args.top);

    println!("Directory: {}", args.path.display());
    println!("Total size: {}", format_size(report.total_size, DECIMAL));
    println!("-------------------------------------------");

    println!("Top {} heaviest files:", args.top);
    println!("");

    for (i, f) in report.top_files.iter().enumerate() {
        println!(
            "{}. {} — {}",
            i + 1,
            f.path.display(),
            format_size(f.size, DECIMAL)
        )
    }
}
