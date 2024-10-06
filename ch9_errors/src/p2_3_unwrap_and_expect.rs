use std::fs::File;

#[allow(dead_code, unused_variables)]
fn main() {
    // unwrap
    // - возвращает значение
    // - вызывает panic!
    let greeting_file = File::open("hello.txt").unwrap();

    // expect аналогично unwrap, но
    // позволяе указать сообщение
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
