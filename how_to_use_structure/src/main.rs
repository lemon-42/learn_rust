// This function takes two separate parameters (width and height), but it doesn't clearly
// indicate that they are related to describe a rectangle.
fn area_parameters(width: u32, height: u32) -> u32 {
    width * height
}

// Wrapping the arguments in a tuple groups them, but accessing tuple elements by index
// (dimensions.0, dimensions.1) is less readable.
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Using a struct is more readable because it explicitly describes the dimensions of a rectangle.
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    println!("=== Area function with parameters ===\n");

    let width1 = 50;
    let height1 = 30;

    println!(
        "The are of the rectangle is : {}",
        area_parameters(width1, height1)
    );

    println!();
    println!("=== Area function with tuple ===\n");

    let area1 = (50, 30);

    println!("The are of the rectangle is : {}", area_tuple(area1));

    println!();
    println!("=== Area function with a structure ===\n");

    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("The area of the rectangle is : {}", area_struct(&rect1));

    // We can easily debug and print the entire rectangle using `Debug` trait.
    let rect1 = Rectangle {
        width: 55,
        height: 30,
    };

    println!("rect1 is : {:#?}", rect1);
}
