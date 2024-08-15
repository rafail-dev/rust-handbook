enum Message {
    Quit,
    Move { _x: i32, _y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &str {
        match self {
            Message::Quit => "Quit",
            _ => "Another",
        }
    }
}

// или

struct _QuitMesage;
struct _MoveMessage {
    x: i32,
    y: i32,
}

struct _WriteMesage(String);

struct _ChangeColorMessage(i32, i32, i32);

pub fn main() {
    let message1 = Message::Quit;

    let message2 = Message::Move { _x: 1, _y: 2 };

    println!(
        "Message 1 - {}; Message 2 - {}",
        message1.call(),
        message2.call()
    );
}

// нельзя сделать также с рядом структур
fn _f1(_: Message) {}
