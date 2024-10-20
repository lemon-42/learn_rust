fn fahr_to_celcius(fahr: f64) -> f64 {
    // Formula : C = T(in fahr) - 32 * 5 / 9
    
    (fahr - 32.0) * (5.0 / 9.0)
}

fn celcius_to_fahr(celcius: f64) -> f64 {
    // Formula : F = T(in celcius) * (9 / 5) + 32

    celcius * (9.0 / 5.0) + 32.0
}

fn main() {
    let first_fahr: f64 = 80.0;
    let second_fahr: f64 = 10.0;

    let first_celcius: f64 = 20.0;
    let second_celcius: f64 = 5.0;

    println!("{first_fahr} fahrenheit in celcius is : {:.2} celcius", fahr_to_celcius(first_fahr));
    println!("{second_fahr} fahrenheit in celcius is : {:.2} celcius", fahr_to_celcius(second_fahr));


    println!("{first_celcius} celcius is : {} fahrenheit", celcius_to_fahr(first_celcius));
    println!("{second_celcius} celcius is : {} fahrenheit", celcius_to_fahr(second_celcius));
} 
