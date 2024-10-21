// Reference are immutable by default


// Without reference (wof)
// We need to return the string to use it afterward since ownership is transferred
#[allow(dead_code)]
fn calculate_length_wof(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

// With reference `&`
// Here, we don't need to return the string since we are borrowing it via a reference `&`
fn calculate_length(s: &String) -> usize {
    s.len()
}

// The following function will not work since we modify the reference
// We cannot mutate a value through an immutable reference `&`
// fn change(text: &String) {
//     text.push_str(", world")
// }

// This will instead work because we pass a mutable reference so we can change it
fn change_mut(text: &mut String) {
    text.push_str(", world")
}

// Uncommenting the following line result in an error.
// Rust compiler guarantees that reference will never be dangling references
// A dangling reference occurs when we refer to memory that has been freed or reallocated.
// fn dangling() -> &String {
//     let s = String::from("hello");
//
//     &s // This would return a reference to a local variable, which is not allowed.
// }


fn main() {
    let s1 = String::from("hello");
    let long = calculate_length(&s1);

    println!("The size of '{s1}' is {long}");

    // We need to make s2 mutable because it will be changed by change_mut
    let mut s2 = String::from("Hello");
    change_mut(&mut s2);

    println!("{s2}");

    // Uncommenting the following line result in an error.
    // We can only have one mutable reference for each data at the same time
    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s; // Error: cannot borrow `s` as mutable more than once
    //
    // println!("{r1} {r2}")
    //
    // However, we can have multiple immutable references at the same time. 
    //
    // let r1 = &s;
    // let r2 = &s;
}
