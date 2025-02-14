use colored::Colorize;
use std::fs::FileType;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct FileInformation {
    path: PathBuf,
    size: u64,
    is_directory: bool,
}

pub fn walk_directories(dir: &str) -> io::Result<()> {
    let mut file_vec = Vec::<FileInformation>::new();

    for entry in WalkDir::new(dir) {
        let raw_entry = entry?;

        //intermediates need to be created, as `into_path()` will consume raw_entry
        let entry_type: FileType = raw_entry.file_type();
        let entry_metadata = raw_entry.metadata();

        let found_file = FileInformation {
            path: raw_entry.into_path(),
            size: entry_metadata?.len(),
            is_directory: entry_type.is_dir(),
        };

        file_vec.push(found_file);
    }

    for x in &file_vec {
        // Apply color depending on whether the entry is a directory or a file
        if x.is_directory {
            // Directories can be printed in blue
            println!("{}, size {}", x.path.display().to_string().blue(), x.size);
        } else {
            // Files can be printed in green
            println!("{}, size {}", x.path.display().to_string().green(), x.size);
        }
    }

    Ok(())
}
