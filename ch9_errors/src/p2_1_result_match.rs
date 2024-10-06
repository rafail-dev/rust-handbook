use std::fs::File;
use std::io::{ErrorKind, Read};

#[allow(dead_code, unused_variables)]
fn main() {
    {
        let greeting_file_result = File::open("hello.txt");

        let mut greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };

        let mut contents = String::new();

        match greeting_file.read_to_string(&mut contents) {
            Ok(_) => println!("File contents:\n{}", contents),
            Err(error) => panic!("Problem reading the file: {error:?}"),
        };
    }

    {
        // попытаться открыть файл,
        // а в случае ошибки "NotFound" попытаться создать файл
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}");
                }
            },
        };
    }
}
