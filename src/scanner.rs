use colored::Colorize;
use std::io;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub struct FileInformation {
    path: PathBuf,
    size: u64,
    is_directory: bool,
}

pub struct DirectoryScanner {
    files: Vec<FileInformation>,
    scan_complete: bool,
    provided_directory: String,
    include_hidden: bool, 
}

impl DirectoryScanner {
    pub fn new(root: &str, include_hidden: bool) -> DirectoryScanner {
        DirectoryScanner {
            files: Vec::<FileInformation>::new(),
            scan_complete: false,
            provided_directory: String::from(root),
            include_hidden: include_hidden, 
        }
    }

    pub fn scan_directories(&mut self) {
        for entry in WalkDir::new(&self.provided_directory)
    {
            match entry {
                Ok(raw_entry) => {
                    let entry_path = raw_entry.path().to_owned();
                    match self.process_entry(raw_entry) {
                        Ok(file_info) => self.files.push(file_info),
                        Err(e) => {
                            eprintln!("Error processing {}: {}", entry_path.display(), e)
                        }
                    }
                }
                Err(e) => eprintln!("Error accessing path: {}", e),
            }
        }
        self.scan_complete = true;
    }

    fn process_entry(&self, entry: DirEntry) -> io::Result<FileInformation> {
        let entry_type = entry.file_type();
        let entry_metadata = entry.metadata()?;

        Ok(FileInformation {
            path: entry.into_path(),
            size: entry_metadata.len(),
            is_directory: entry_type.is_dir(),
        })
    }

    pub fn print_directories(&mut self) {
        for file_info in &self.files {
            // Apply color depending on whether the entry is a directory or a file
            if file_info.is_directory {
                // Directories can be printed in blue
                println!(
                    "{}, size {}",
                    file_info.path.display().to_string().blue(),
                    file_info.size
                );
            } else {
                // Files can be printed in green
                println!(
                    "{}, size {}",
                    file_info.path.display().to_string().green(),
                    file_info.size
                );
            }
        }
    }
    // fn is_hidden(entry: &DirEntry) -> bool {
    //     // Only check the file name itself, not the full path
    //     entry.file_name()
    //         .to_str()
    //         .map(|s| s != "." && s != ".." && s.starts_with("."))
    //         .unwrap_or(false)
    // }
}
