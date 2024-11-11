pub fn control_flow() 
{
    let condition = false;
    
    let number = if condition { 5 } else { 50};
    println!("The number is {}", number);
    
    
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


fn fahrenheit_to_celsius(fahrenheit: f64) -> f64
{
    (fahrenheit - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(x: f64) -> f64{
    (x*(9.0/5.0))+32.0
}

fn fibonacci(n: u32) -> u32{
    if n<=1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}