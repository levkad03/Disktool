use std::path::PathBuf;

use clap::Parser;
use disktool::scan_dir;
use humansize::{DECIMAL, format_size};

#[derive(Parser)]
struct Args {
    // Path to the directory
    path: PathBuf,
    // Top heaviest files to show
    #[arg(long, default_value = "10")]
    top: usize,
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
