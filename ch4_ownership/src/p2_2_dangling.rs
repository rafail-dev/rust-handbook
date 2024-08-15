pub fn main() {}

// s уже будет недоступна после завершения работы функции

// cannot return reference to local variable `s`
// returns a reference to data owned by the current function
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
