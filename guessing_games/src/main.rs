use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number !");

    // Initialize a random number generator (RNG) specific to the current thread
    // This RNG is automatically seeded. 
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");

    // Initialize an empty string to store the guess
    let mut guess = String::new();

    // Reads input from stdin and store it in the mutable variable guess
    // The `read_line()` require a mutable reference (`&`) so it can modify the variable
    // Since the `read_line()` return a `Result`, we handle any potential error using `expect()`
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed : {}", guess);
}
