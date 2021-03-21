fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{:?}", plus_one(None));
    println!("{}", plus_one(Some(7)).unwrap());
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
