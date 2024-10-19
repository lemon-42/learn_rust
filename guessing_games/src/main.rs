use std::io;

fn main() {
    println!("Guess the number !");

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
