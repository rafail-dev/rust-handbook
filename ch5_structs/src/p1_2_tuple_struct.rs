#[allow(dead_code)]
struct Point(i32, i32, i32);

// Unit-like структура
struct AlwaysEqual;

pub fn main() {
    let _point1 = Point(1, 2, 3);

    let _subject = AlwaysEqual;
}
