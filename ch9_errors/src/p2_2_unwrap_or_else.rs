use std::fs::File;
use std::io::ErrorKind;

#[allow(dead_code, unused_variables)]
fn main() {
    // аналогично p2_1
    // попытаться открыть файл,
    // а в случае ошибки "NotFound" попытаться создать файл

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
