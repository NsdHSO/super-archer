pub fn try_understand_ownership(){
    let some_string = String::from("hello");
    let some_string_ref = some_string;
    iancu(&some_string_ref);
    
    println!("{some_string_ref}")
    
}

fn iancu(s: &String)  {
    println!("{}", s);
}