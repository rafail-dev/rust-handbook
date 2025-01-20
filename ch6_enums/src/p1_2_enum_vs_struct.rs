#![allow(dead_code, unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &str {
        match self {
            Message::Quit => "Quit",
            _ => "Another",
        }
    }
}

pub fn main() {
    let message1 = Message::Quit;

    let message2 = Message::Move { x: 1, y: 2 };

    println!(
        "Message 1 - {}; Message 2 - {}",
        message1.call(),
        message2.call()
    );
}
