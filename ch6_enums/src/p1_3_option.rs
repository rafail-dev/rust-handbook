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
    let _some_number = Some(1);
    let _some_char = Some('a');

    // compiler can't infer type
    let mut _absent_number: Option<i32> = None;
    _absent_number = Some(1);

    // Нельзя сложить Option<i32> с Option<i32> или c i32,
    // компилятору необходимл доказать, что значение есть
    // let sum = _some_number + _absent_number;
    let result = add(_some_number, _absent_number);
    println!("{:?}", result)
}
