use std::path::{PathBuf, Path};
use walkdir::{WalkDir, DirEntry};

pub struct FileInformation {
    path: PathBuf,
    size: u64,
    is_directory: bool
}
