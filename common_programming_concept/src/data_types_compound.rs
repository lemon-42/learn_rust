#[allow(unused_variables)]

pub fn compound() {
    /*          Tuple type              */
    // Can hold multiple differents types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // To get the individual values out of a tuple
    // We can use pattern matching to destructure a tuple value
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    // We can access a element in a tuple by using a '.' with the index of the value
    let x: (i32, f64, u8) = (500, 6.4, 1);
    
    let five_hundred = x.0; // 500
    let six_point_four = x.1; // 6.4
    let one = x.2; // 1

    /*          Array type              */
    // Unlike a tuple, every element of an array must have the same type
    let a = [1, 2, 3, 4, 5];
    let first = a[0]; // 1
    let second = a[1]; // 2

    // An array is really handy when we know it's size in advance, unlike vectors
    // Which can grow as the program runs
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // Writting an array using explicit annotation
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    let c = [3; 5]; // Contains : [3, 3, 3, 3, 3];
}
