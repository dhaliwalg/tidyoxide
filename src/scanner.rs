use std::path::{PathBuf, Path};
use walkdir::{WalkDir, DirEntry};

pub struct FileInformation {
    path: PathBuf,
    size: u64,
    is_directory: bool
}

fn walk_directories() {
    for entry in WalkDir::new(".") {
        println!("{}", entry?.path().display());
    }
}