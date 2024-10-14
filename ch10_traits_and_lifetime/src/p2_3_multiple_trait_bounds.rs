#![allow(unused, dead_code)]

use std::{ops::Add, path::Display};

// The Copy trait requires the type to also implement the Clone trait
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

trait Distance {
    fn distance_from_source(&self) -> f64;
}

impl Distance for Point {
    fn distance_from_source(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Add<Output = T>
fn foo<T: Distance + Add<Output = T> + Copy>(p: T) -> f64 {
    let x = p + p + p;
    x.distance_from_source()
}

fn main() {
    let p1 = Point { x: 3, y: 4 };
    println!("p3: {:?}", p1);

    let p2 = Point { x: 1, y: 1 };

    // Without implementing the Copy trait, it would not be possible to add p1 to p1,
    // because the ownership of p1 would be moved to the add function.
    let p3 = p1 + p1 + p2 + p2;

    let r = foo(p3);
    println!("r: {}", r);
}
