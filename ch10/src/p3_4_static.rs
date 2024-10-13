#![allow(unused, dead_code)]

fn main() {
    let implicitly: &str = "Hello";

    // affected reference can live for the entire duration of the program
    // text of this string is stored directly in the program’s binary, which is always available
    let explicitly: &'static str = "Hello";
}
