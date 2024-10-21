#[allow(unused_variables)]

// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection
// A slice is a kind of reference, so it does not have ownership.

// A small programming problem
// Write a function that takes a string of words separated by spaces and returns the first word it finds in that string
// If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

// Without slices
// This function takes a reference to a String since we only need to borrow it.
// It returns the index of the first space, indicating the end of the first word.
fn first_word_wos(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Using slices
// This version uses `&str` instead of `&String` because `&str` works for both string literals and String types.
// It returns a slice of the string representing the first word.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

fn main() {
    let s1 = String::from("hello my name is lemon");
    let indexof = first_word_wos(&s1);
    println!("{indexof}");

    let s2 = String::from("hello my name is lemon");
    let indexof = first_word(&s2);
    println!("{indexof}");

    // This is a character slice
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // This works
    let slice = &s[..2];

    // This also works
    let len = s.len();
    let slice = &s[5..len];
    let slice = &s[5..];

    // No limit slice
    let slice = &s[0..len];
    let slice = &s[..];
}
