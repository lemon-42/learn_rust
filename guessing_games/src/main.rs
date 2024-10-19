use std::io;
use std::cmp::Ordering;
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

    // Shadow the old `guess` variable with this new one
    // First, we use `trim()` to remove all trailing character and whitespaces
    // Then we parse it into another type, here u32 to match with `secret_number` variable
    // As `parse()` return a `Result`,we handle any potential error using `expect()`
    let guess = guess.trim().parse::<u32>().expect("Please enter a number !");

    println!("You guessed : {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("It's more !"),
        Ordering::Greater => println!("It's less !"),
        Ordering::Equal => println!("You win !"),
    }
}
