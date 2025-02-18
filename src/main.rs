mod scanner;
use scanner::DirectoryScanner;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a directory
    Scan {
        /// Directory to scan
        directory: String,
        
        /// Include hidden files
        #[arg(long, default_value_t = false)]
        hidden: bool,
    },
}
fn main() {
    // println!("Hello, world!");
    // println!("I'll list all files in the provided directory for organization.");
    // println!("What directory would you like to scan?");
    // let mut directory = String::new();

    // io::stdin()
    //     .read_line(&mut directory)
    //     .expect("error :(");

    // //need trim b/c newline stored on ENTER
    // let directory = directory.trim();
    // let mut dir_scan = DirectoryScanner::new(directory);
    // let _ = dir_scan.scan_directories();
    // dir_scan.print_directories();
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { directory, hidden } => {
            let mut dir_scan = DirectoryScanner::new(&directory, hidden);
            let _ = dir_scan.scan_directories();
            dir_scan.print_directories();
        }

        
    }
}