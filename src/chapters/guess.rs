use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess() {
    let secret_number = rand::thread_rng().gen_range(0..=1000);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(numb) => numb,
            Err(_) => continue,
        };
        println!("Please input your guess.");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("You guessed: {guess}");
    }
}
