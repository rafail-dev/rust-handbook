#![allow(dead_code, unused)]

struct Circle {
    radius: u32,
}

// ассоциированные функции,
// которым не нужен экземпляр для работы
impl Circle {
    fn new(radius: u32) -> Circle {
        Circle { radius }
    }

    fn not_self(a: u8, b: u8) {
        println!("{} {}", a, b);
    }
}

// можно определить несколько блоков impl
// если ф-я повторится, то будет ошибка компиляции
impl Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * (self.radius as f64)
    }
}

fn main() {
    let circle1 = Circle::new(30);

    // found the following associated functions; to be used as methods, functions must have a `self` parameter
    // circle1.not_self(1,1);
    Circle::not_self(1, 1);

    println!("Perimeter of a Circle 1 - {}", circle1.perimeter());
}
