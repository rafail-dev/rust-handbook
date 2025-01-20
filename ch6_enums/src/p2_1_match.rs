#![allow(dead_code, unused)]

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
        // каждая ветка возвращает expression
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // при сопоставлении с образцом
        // можно извлекать значения
        Coin::Quarter(state) => 25,
    }
}

fn incremented(x: Option<i32>) -> Option<i32> {
    match x {
        Some(v) => Some(v + 1),
        None => None,
    }
}

fn _incremented_if_one_or_seven(x: i32) -> Option<i32> {
    match x {
        1 => Some(2),
        7 => Some(8),
        another_value => None,
        // если значение неважно, то можно использовать _
        // _ => None,

        // компилятор гарантирует удовлетворение
        // требования исчерпывающей полноты
    }
}

pub fn main() {
    let coin1 = Coin::Dime;
    let coin2 = Coin::Quarter(UsState::Alabama);

    println!("Sum = {}", value_in_cents(coin1) + value_in_cents(coin2));
    println!("Incremented - {:?}", incremented(Some(1)));
    println!("Incremented - {:?}", incremented(None));
}
