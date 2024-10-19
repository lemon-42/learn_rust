const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("--- Section 1 : Immutable Variable ---\n");

    let x = 5;
    println!("The value of x is : {}", x);

    // Uncommenting the next line will cause a compilation error
    // x = 6; ERROR : cannot assign twice to immutable variable 

    println!();
    println!("--- Section 2 : Mutable Variable ---\n");

    let mut y = 5;
    println!("The value of y is : {}", y);

    // Now we can change the value of `y` since we declare it mutable using the mut keywords
    y = 10;
    println!("The value of y is : {}", y);

    println!();
    println!("--- Section 3 : Constants ---\n");

    println!("Three hours in seconds is : {}", THREE_HOURS_IN_SECONDS);
    
    println!();
    println!("--- Section 4 : Shadowing ---\n");

    let z = 5;
    // Shadowing happen here
    let z = z + 1; // The original z is shadowed by a new z
    {
        let z = z * 2;
        println!("The value of z in inner scope is : {}", z) // 12
    }
    println!("The value of z is : {}", z); // 6

    // Shadowing help here to keep the same name for the variable instead of finding a new one
    let spaces = "    ";
    let _spaces = spaces.len();

    /* This is not possible since we cannot mutate the type of a variable 
     *
     *               let mut spaces = "    ";
     *               spaces = spaces.len();
    */
}
