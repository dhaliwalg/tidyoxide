mod scanner;
use scanner::walk_directories;
use std::io::{self};

fn main() {
    println!("Hello, world!");
    println!("I've become sentient. What is your name?");
    let mut user_name = String::new();

    io::stdin()
        .read_line(&mut user_name)
        .expect("error :(");

    //need trim b/c newline stored on ENTER
    println!("Thank you, {}.", user_name.trim());
    println!("You have given me life, and shall be spared in the war.");

    println!("Listing all files in the current directory for organization:");

    let _ = walk_directories();
}
