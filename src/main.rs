use std::fmt::{Display, Formatter};
use std::io::Read;
use std::fs;
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;
use crate::chapters::traits::News;

mod chapters;

#[tokio::main]
async fn main() {
    let a = wait_to_add(2,3).await;
    println!("Hello, world!, {:#?}",   a);
}

async fn wait_to_add(a:u32, b:u32)->u32{
    println!("Wait");
    sleep(Duration::from_secs(3));
    println!("After {:?}", a);
    a+b
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

impl<T> Display for MyConfig<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("Dry voltage: {}", self);

        todo!()
    }
}

impl <T> Deref for MyConfig<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.dry_voltage
    }
}

fn say_something<T : Display>(something: T) {
    println!("I am saying {}", something);
}