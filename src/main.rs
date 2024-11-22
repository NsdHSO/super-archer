use std::fs;
use std::fs::File;
use std::io::Read;

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

    match read_username_from_file() {
        Ok(username) => println!("{}", username),
        Err(_) => println!("No username found"),
    }
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let username_file_result = File::open("src/username.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}
