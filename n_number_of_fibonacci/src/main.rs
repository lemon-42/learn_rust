fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let mut number = 1;

    while number <= 15 {
        println!("Fibonacci({number}) is {}", fibonacci(number));
        number += 1;
    }
}

