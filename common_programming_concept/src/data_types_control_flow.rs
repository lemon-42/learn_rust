#[allow(unused_variables)]

pub fn control_flow() {
    println!();
    println!("More about control flow\n");
    /*          if expression              */
    let number = 5; 

    // Match can be more readable here, more on that later
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 3 }; // 5
    
    // Type must be the same when using a if statements
    // The following lines will report an error due to invalid type
    //let number = if condition { 5 } else { "three" };
    
    println!("The value of number is : {number}");

    /*          loop expression              */
    
    // Uncommenting the next line will result in a infinite loop
    //loop {
    //    println!("Again and again..");
    //}
    
    // Returning value of a loop 
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is : {}", result); // 20

    /*           while expression         */
    let mut number = 3;

    while number != 0 {
        println!("{number} !");

        number -= 1;
    }

    println!("LIFTOFF!");

    /*           for expression         */
    let a = [10, 20, 30, 40, 50];

    for elem in a {
        println!("{elem}");
    }

    for number in (1..=3).rev() {
        println!("{number} !")
    }

    println!("LIFTOFF!");
}

