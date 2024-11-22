use std::fs::File;
use std::io::Read;
use std::{fs, io};

mod chapters;

fn main() {
    let file = fs::read("src/text.txt");
    let file = match file {
        Ok(file) => file,
        Err(err) => panic!("No file {}", err),
    };

    for line in file.split(|&byte| byte == b'\n') {
        match String::from_utf8(line.to_vec()) {
            Ok(string) => println!("{}", string),
            Err(err) => panic!("{}", err),
        }
    }
    // Step 1: Read the existing file content
    let mut content = fs::read_to_string("src/username.txt")
        .unwrap_or_else(|_| String::new()); // If the file doesn't exist, start with an empty string

    
    

    io::stdin()
        .read_line(&mut content)
        .expect("Failed to read line");

    fs::write("src/username.txt", &content).expect("Unable to write file");
}
