// Mod file to have rust-analyzer working on those file
// More about this later
mod variables;
mod data_types_scalar;
mod data_types_compound;

fn main() {
    println!("Let's learn more about common programming concept!\n");

    // The following is used to not have compilation error
    variables::variables();

    data_types_scalar::scalar();
    data_types_compound::compound();
}
