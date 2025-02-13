use std::path::{PathBuf, Path};
use walkdir::{WalkDir, DirEntry};
use std::io;

pub struct FileInformation {
    path: PathBuf,
    size: u64,
    is_directory: bool
}

pub fn walk_directories() -> io::Result<()> {
    for entry in WalkDir::new(".") {
        let entry = entry?;
        println!("{}", entry.path().display());  // Print the path of each entry
    }
    Ok(())
}