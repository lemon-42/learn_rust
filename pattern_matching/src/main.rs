#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("It's from : {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }

    // This is also possible
    // x.map(|x| x + 1)
}

fn add_cool_hat() {}
fn remove_cool_hat() {}
fn move_player(x: u8) {}
fn reroll() {}

fn main() {
    println!("Let's learn more about pattern matching !");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{five:?} | {six:?} | {none:?}");

    let dice_roll = 9;
    match dice_roll {
        3 => add_cool_hat(),
        7 => remove_cool_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_cool_hat(),
        7 => remove_cool_hat(),
        _ => reroll(),
    }
}
