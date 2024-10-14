#![allow(unused, dead_code)]

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: T,
    meta: U,
}

impl<U> Point<f32, U> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let point1 = Point {
        x: 10.0,
        y: 10.0,
        meta: "comment",
    };

    let point2 = Point {
        x: 10,
        y: 20,
        meta: true,
    };
}
