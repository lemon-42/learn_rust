// Mod file to have rust-analyzer working on those file
// More about this later
mod variables;
mod data_types;

fn main() {
    println!("Let's learn more about common programming concept!\n");

    // The following is used to not have compilation error
    println!("More about variables.\n");
    variables::variables();

    println!("\nMore about data types.\n");
    data_types::data_types();
}
