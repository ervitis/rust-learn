
fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{:?}", plus_one(None));
    println!("{}", plus_one(Some(7)).unwrap());

    let some_value = Some(0u8);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    };

    let coin = Coin::Quarter(String::from("Alabama"));

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("{} from state", state);
            1
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
