#![allow(unused, dead_code)]

pub struct My1 {
    pub x: i32,
    pub y: i32,
}

pub struct My2 {
    a: i32,
    b: i32,
}

pub trait MyTrait {
    fn my_f(&self) -> i32;
    fn with_default() -> i32 {
        0 // 0 is default
    }
}

impl MyTrait for My1 {
    fn my_f(&self) -> i32 {
        self.x * 2
    }
}

impl MyTrait for My2 {
    fn my_f(&self) -> i32 {
        self.a * 22
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

// outside
// use p2_trait::{My1, MyTrait};
fn main() {}
