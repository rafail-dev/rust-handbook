#![allow(dead_code, unused)]
struct Point(i32, i32, i32);

// Unit-like структура
struct AlwaysEqual;

pub fn main() {
    let point1 = Point(1, 2, 3);

    let subject = AlwaysEqual;
}
