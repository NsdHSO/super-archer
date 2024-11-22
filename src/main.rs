use std::collections::HashMap;
use crate::chapters::enums::IpAddr;

mod chapters;

fn main() {
    let field_one= String::from("field_one");
    let field_three= String::from("field_three");
    let field_two= String::from("field_two");
    
    let mut has_one= HashMap::new();
    has_one.insert(field_one.clone(), field_two);
    
    println!("{:}", field_one);
    println!("{:}", has_one.get(&field_three).unwrap_or( &String::from("fsaf")));
}
