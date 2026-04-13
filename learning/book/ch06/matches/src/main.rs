fn main() {
    let penny = Coin::Penny;
    println!("Un penny vale {} centavo", value_in_cents(penny));

    let nickel = Coin::Nickel;
    println!("Un nickel vale {} centavos", value_in_cents(nickel));

    let dime = Coin::Dime;
    println!("Un dime vale {} centavos", value_in_cents(dime));

    let quarter = Coin::Quarter(UsState::Alaska);
    println!("Un quarter vale {} centavos", value_in_cents(quarter));

    let hometown_quarter = Coin::Quarter(UsState::Alabama);
    println!(
        "Otro quarter vale {} centavos",
        value_in_cents(hometown_quarter)
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("plus_one(Some(5)) -> {:?}", six);
    println!("plus_one(None) -> {:?}", none);
}

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
            println!("Moneda de 1 centavo.");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter del estado de {:?}.", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1),
    }
}
