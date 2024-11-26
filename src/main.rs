use std::io::Read;
use std::fs;
use std::ops::Deref;
use crate::chapters::traits::News;

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
    let post = Post{
        title: "Parandulea".to_string(),
        author: "Andrea".to_string(),
        comment: "Iancu is the best".to_string(),
        content,
    };
    let dry_voltage = 5;
    let my_config = MyConfig::new(dry_voltage,10);
    
    
    println!("{}", *my_config);   
    println!("{}", Post::comments());
}
#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    comment: String,
    content: String,
}

impl News for Post {
    fn read(self) -> Post{
        self
    }

    fn comments() -> String {
        String::from("Iancu")
    }
}

struct MyConfig<T> {
    dry_voltage: T,
    wet_voltage: T,
    moisture_percentage: T,
}

impl<T: Default> MyConfig<T> {
    fn new(dry_voltage: T, wet_voltage: T) -> MyConfig<T> {
        MyConfig{
            dry_voltage,
            wet_voltage,
            moisture_percentage: T::default()
        }
    }
}

impl <T> Deref for MyConfig<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.dry_voltage
    }
}