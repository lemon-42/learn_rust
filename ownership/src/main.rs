#[allow(unused_variables)]

fn main() {
    /*
     {                     // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    */

    // This is allocated on the heap
    let s = String::from("hello");

    // This type of string can be mutable
    let mut ss = String::from("hello");
    ss.push_str(", world");
    println!("{ss}");

    /*
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    */

    // Uncommenting the following line result in an error. 
    // Since both variables would point to the same memory address, Rust prevents a double free error.
    // When `s1` goes out of scope, it would try to free the same memory twice, leading to memory safety issues.
    // let s1 = String::from("hello");
    // let s2 = s1;

    // Instead, we can use `clone` to perform a deep copy, creating a completely separate copy of the data.
    // However, cloning can be expensive in terms of performance and memory, so use it carefully.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1} | {s2}");

    // The following implement the `Copy` trait. This means that a variable is always effective
    // after being assigned to another variable (no need to clone) : 
    //
    // All `integer` type
    // Boolean type
    // Floating-point types
    // Character type
    // Tuple, only if they contain type that implement the `Copy` trait. 
    // So (i32, i32) this works and this (i32, String) doesn't.

}
