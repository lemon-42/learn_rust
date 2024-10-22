// This function takes two separate parameters (width and height), but it doesn't clearly
// indicate that they are related to describe a rectangle.
//fn area_parameters(width: u32, height: u32) -> u32 {
//    width * height
//}

// Wrapping the arguments in a tuple groups them, but accessing tuple elements by index
// (dimensions.0, dimensions.1) is less readable.
//fn area_tuple(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}

// Using a struct is more readable because it explicitly describes the dimensions of a rectangle.
//fn area_struct(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// All methods here are related to the Rectangle struct.
// `self` refers to the instance of the Rectangle being used.
// We only take a reference of the structure since we just want to read the data
// Not to modify them. To do that, we could have used `&mut self`
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // Methods can have the same name as a field in the structure
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is an *associated function* because it does not take `self` as a parameter.
    // It's related to the Rectangle struct but does not operate on a specific instance.
    // Associated functions are called using `::` rather than `.`.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /*
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
    */
    println!("=== Area function with a structure ===\n");

    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("The area of the rectangle is : {}", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is : {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 30,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect3 hold rect1? {}", rect3.can_hold(&rect1));

    let square = Rectangle::square(5);
    println!("square is : {:#?}", square);

    //println!("The area of the rectangle is : {}", area_struct(&rect1));

    // We can easily debug and print the entire rectangle using `Debug` trait.
    //let rect1 = Rectangle {
    //    width: 55,
    //    height: 30,
    //};

    //println!("rect1 is : {:#?}", rect1);
}
