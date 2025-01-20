#![allow(dead_code, unused)]
// уже включено в Prelude
// enum Option<T> {
//     None,
//     Some(T),
// }

fn add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(va), Some(vb)) => Some(va + vb),
        _ => None,
    }
}

pub fn main() {
    // compiler can infer type
    let some_number = Some(1);
    let some_char = Some('a');

    // compiler can't infer type without using
    let mut absent_number = None;
    absent_number = Some(1);

    // Cannot add Option<i32> with Option<i32> or with i32,
    // needs to prove that the values exists
    // let sum = some_number + absent_number;
    let result = add(some_number, absent_number);
    println!("{:?}", result)
}
