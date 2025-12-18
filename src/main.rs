use clap::{Parser, Subcommand};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "file_organizer")]
#[command(about = "Scan and organize files in a folder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print a summary of files in a folder
    Scan {
        /// Folder to scan
        folder: PathBuf,
    },

    /// Organize files into subfolders by extension
    Organize {
        /// Folder to organize
        folder: PathBuf,

        /// Show what would happen without moving files
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { folder } => match scan_folder(&folder) {
            Ok(report) => print_report(&folder, &report),
            Err(e) => {
                eprintln!("Error scanning {:?}: {}", folder, e);
                std::process::exit(1);
            }
        },

        Commands::Organize { folder, dry_run } => {
            if let Err(e) = organize_by_extension(&folder, dry_run) {
                eprintln!("Error organizing {:?}: {}", folder, e);
                std::process::exit(1);
            }
        }
    }
}

struct Report {
    total_entries: usize,
    files: usize,
    dirs: usize,
    by_extension: BTreeMap<String, usize>,
}

fn scan_folder(folder: &PathBuf) -> std::io::Result<Report> {
    let mut report = Report {
        total_entries: 0,
        files: 0,
        dirs: 0,
        by_extension: BTreeMap::new(),
    };

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        report.total_entries += 1;

        let path = entry.path();
        let meta = entry.metadata()?;

        if meta.is_dir() {
            report.dirs += 1;
            continue;
        }
        if meta.is_file() {
            report.files += 1;

            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "(no_ext)".to_string());

            *report.by_extension.entry(ext).or_insert(0) += 1;
        }
    }

    Ok(report)
}

fn print_report(folder: &PathBuf, report: &Report) {
    println!("Folder: {:?}", folder);
    println!("Total entries: {}", report.total_entries);
    println!("Files: {}", report.files);
    println!("Dirs: {}", report.dirs);
    println!("\nFiles by extension:");

    if report.by_extension.is_empty() {
        println!("  (none)");
        return;
    }

    for (ext, count) in &report.by_extension {
        println!("  {:>8}  {}", count, ext);
    }
}

fn organize_by_extension(folder: &Path, dry_run: bool) -> std::io::Result<()> {
    // Safety: only organize files in the top-level of `folder` (no recursion).
    // Create subfolders like: organized/txt, organized/png, organized/no_ext
    let organized_root = folder.join("organized");

    let mut moves: Vec<(PathBuf, PathBuf)> = Vec::new();

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        // Skip the "organized" folder itself and any directories.
        if path.file_name().and_then(|n| n.to_str()) == Some("organized") {
            continue;
        }
        let meta = entry.metadata()?;
        if !meta.is_file() {
            continue;
        }

        let ext_folder = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| "no_ext".to_string());

        let dest_dir = organized_root.join(ext_folder);
        let file_name = path.file_name().unwrap(); // safe: it's a file path
        let dest_path = dest_dir.join(file_name);

        moves.push((path, dest_path));
    }

    if moves.is_empty() {
        println!("No files to organize in {:?}", folder);
        return Ok(());
    }

    if dry_run {
        println!("Dry run: planned moves");
        for (src, dst) in &moves {
            println!("  {:?} -> {:?}", src, dst);
        }
        println!("\nNothing was moved (dry-run).");
        return Ok(());
    }

    // Real move
    for (src, dst) in moves {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&src, &dst)?;
        println!("Moved {:?} -> {:?}", src, dst);
    }

    println!("\nDone. Files organized into {:?}", organized_root);
    Ok(())
}