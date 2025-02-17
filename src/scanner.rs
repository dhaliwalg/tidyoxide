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

pub struct DirectoryScanner {
    files: Vec::<FileInformation>,
    scan_complete: bool,
    provided_directory: String,
}

impl DirectoryScanner {
    pub fn new(root: &str) -> DirectoryScanner {
        DirectoryScanner {
            files: Vec::<FileInformation>::new(),
            scan_complete: false,
            provided_directory: String::from(root),
        }
    }

    pub fn scan_directories(&mut self) -> io::Result<()> {
        for entry in WalkDir::new(&self.provided_directory) {
            let raw_entry = entry?;
    
            //intermediates need to be created, as `into_path()` will consume raw_entry
            let entry_type: FileType = raw_entry.file_type();
            let entry_metadata = raw_entry.metadata();
    
            let found_file = FileInformation {
                path: raw_entry.into_path(),
                size: entry_metadata?.len(),
                is_directory: entry_type.is_dir(),
            };
    
            &self.files.push(found_file);
        }
    
        for file_info in &self.files {
            // Apply color depending on whether the entry is a directory or a file
            if file_info.is_directory {
                // Directories can be printed in blue
                println!("{}, size {}", file_info.path.display().to_string().blue(), file_info.size);
            } else {
                // Files can be printed in green
                println!("{}, size {}", file_info.path.display().to_string().green(), file_info.size);
            }
        }
        Ok(())
    }
}