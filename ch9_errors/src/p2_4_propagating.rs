use std::fs::{self, File};
use std::io::{self, Read};

#[allow(dead_code, unused_variables)]
fn main() {
    println!("{}", read_username_from_file_verbose().unwrap());
    println!("{}", read_username_from_file_concise1().unwrap());
    println!("{}", read_username_from_file_concise2().unwrap());
    println!("{}", read_username_from_file_concise3().unwrap());

    println!("{}", last_char_of_first_line("hello\na").unwrap())
}

fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    // return означает выход
    // из функции read_username_from_file
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_concise1() -> Result<String, io::Error> {
    // оператор - ?
    // при Err
    // - прекратит работу функции
    // - вернет Err

    // для ошибки вызывается from из трейта From для
    // для преобразования типа Err в указанный тип ошибки
    // в возвращаемом типе текущей функци
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // ? можно использовать в функциях с возвращаемым типом,
    // который реализует FromResidual (Result, Option ...)
}

fn read_username_from_file_concise2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_concise3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
