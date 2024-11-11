pub fn tuple(){
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tuple is: {}", tup.2);
}