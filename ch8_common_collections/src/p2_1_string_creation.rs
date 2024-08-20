#[allow(unused_variables, dead_code)]
pub fn main() {
    // String - это вектор с дополнительным функционалом
    // String можно создать разными способами
    let s = "Hello".to_string();
    let initial_data = "hello";
    let s = initial_data.to_string();
    let s = String::from("Hello");
}
