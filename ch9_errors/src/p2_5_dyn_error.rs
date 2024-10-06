use std::error::Error;
use std::fs::File;

// Box<dyn Error> - любой вид ошибки
// можно возвращать https://doc.rust-lang.org/std/process/trait.Termination.html
// main - на верхнем уровне программы

#[allow(dead_code, unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
