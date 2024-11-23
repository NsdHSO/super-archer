use std::io::Read;
use std::net::Ipv4Addr;
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
    let mut content = fs::read_to_string("src/username.txt").unwrap_or_else(|_| String::new()); // If the file doesn't exist, start with an empty string

    fs::write("src/username.txt", &content).expect("Unable to write file");
    let arrary = [1,2,3,4,5,6,7,8,9,10,1,3,5,100];
    let chars = vec!['y', 'm','z', 'a', 'q'];

    println!("{:?}",largest(&arrary));
    println!("{:?}",largest(&chars));
}

fn my_home(ipAddres: &mut String) -> Ipv4Addr {
    let ip = ipAddres.trim().parse();
    ip.unwrap_or_else(|err| {
        println!("Unable to parse {}", err);
        Ipv4Addr::new(0, 0, 0, 0)
    })
}

fn largest<T : PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}