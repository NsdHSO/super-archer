use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..=1000);
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {println!("You win!");}
    }

    println!("You guessed: {guess}, and The secret number is {secret_number}");
}