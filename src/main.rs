use std::io::{self, Write};
use std::fs::File;

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
    list_directory();
}

fn list_directory() -> std::io::Result<()>{
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}