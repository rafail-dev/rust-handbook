#[allow(dead_code)]
// size = 24 (0x18), align = 0x8
// почему 24?
enum IntOrFloatOrText {
    Int(u8),
    Float(f64),
    Text(String),
}

#[allow(dead_code, unused_variables)]
pub fn main() {
    let v: Vec<IntOrFloatOrText> = vec![
        IntOrFloatOrText::Int(255),
        IntOrFloatOrText::Float(1.1),
        IntOrFloatOrText::Text(String::from("Hello")),
    ];
}
