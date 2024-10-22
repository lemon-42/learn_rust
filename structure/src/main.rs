use structure_examples::area;

mod structure_examples;

// Structure are like tuples. But, unlike tuples, we can give a name to the element
// of our structure. Also, we don't have to rely on the order of the data to specify
// or access values of an instance

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like struct
struct AlwaysEqual;

#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("=== Section 5.1 ===\n");
    // To use a structure after we've defined it, we create an instance of that struct
    // by specifying concrete values for each fields.
    // If we want to change sime fields afterwards, we need to mutate our instance
    let mut user1 = User {
        active: true,
        email: String::from("lemon@world.com"),
        username: String::from("lemon"),
        sign_in_count: 1,
    };

    // To access a fields of our structure, we use the "." notation
    user1.email = String::from("lemon@void.com");

    // Example case of using the build_user function
    // let user2 = build_user(String::from("chicco@void.com"), String::from("chicco"));

    // Example case on how to create a new ... (complete this)
    // let user2 = User {
    //     active: user1.active,
    //     email: String::from("chicco@world.com"),
    //     username: String::from("chicco"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // This work the same as above
    // let user2 = User {
    //     username: String::from("chicco"),
    //     ..user1
    // };

    // Note: The struct update syntax uses `=` to move data.
    // After creating `user2`, we can't use `user1` entirely because the
    // `String` in `username` was moved. If we had assigned new `String`
    // values to `user2`'s fields, only `active` and `sign_in_count` from
    // `user1` would be copied, as they implement the `Copy` trait.
    // So this will not work if we declare user2 with user1 attributes:
    //
    // user1.email = String::from("hello");

    /* Tuple struct */
    let origin = Point(0, 0, 0);
    let black = Color(0, 0, 0);

    /* Unit-Like struct */
    let subject = AlwaysEqual;

    println!("=== Section 5.2 ===");

    let width1 = 50;
    let height1 = 30;

    println!("The area of the rectangle is : {}", area(width1, height1));
}
