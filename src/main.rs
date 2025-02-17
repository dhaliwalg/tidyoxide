mod scanner;
use std::io::{self};

use scanner::DirectoryScanner;

fn main() {
    println!("Hello, world!");
    println!("I'll list all files in the provided directory for organization.");
    println!("What directory would you like to scan?");
    let mut directory = String::new();

    io::stdin()
        .read_line(&mut directory)
        .expect("error :(");

    //need trim b/c newline stored on ENTER
    let directory = directory.trim();
    let mut dir_scan = DirectoryScanner::new(directory);
    let _ = dir_scan.scan_directories();
}
