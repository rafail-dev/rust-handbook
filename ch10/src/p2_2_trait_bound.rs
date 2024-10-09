#![allow(unused, dead_code)]

struct My1 {
    x: i32,
    y: i32,
}

trait MyTrait {
    fn my_f(&self) -> i32;
}

impl MyTrait for My1 {
    fn my_f(&self) -> i32 {
        self.x * 2
    }
}

fn f<T: MyTrait>(p: T) -> i32 {
    p.my_f()
}

// same
fn sugar(p: &impl MyTrait) -> i32 {
    p.my_f()
}

fn main() {
    let my1 = My1 { x: 10, y: 20 };
    println!("{}", f(my1));
}