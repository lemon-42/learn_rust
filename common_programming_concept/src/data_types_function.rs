pub fn for_main() {
    println!();
    println!("More about function\n");

    snake_case_function();

    function_with_arguments(42);

    multiple_argument_function(42, 'L');

    let x = five();
    println!("The value of x is : {x}");

    let y = plus_one(5);
    println!("The value of y is : {y}");
}

// Rust, like a large amount of programming language use snake_case convention when naming function
fn snake_case_function() {
    println!("Snake case function here !");
}

fn function_with_arguments(x: i32) {
    println!("The value of x is : {x}");
}

fn multiple_argument_function(x: i32, unite: char) {
    println!("The multiple argument of this function are {x}{unite}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
