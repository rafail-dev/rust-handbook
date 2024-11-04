#![allow(dead_code, unused)]

use std::cell::RefCell;

#[derive(Debug)]
struct MyStruct(RefCell<u8>);

trait MyTrait {
    fn declared_as_immutable(&self) -> u8;
}

impl MyStruct {
    fn new(v: u8) -> MyStruct {
        MyStruct(RefCell::new(v))
    }
}

impl MyTrait for MyStruct {
    fn declared_as_immutable(&self) -> u8 {
        let mut r = self.0.borrow_mut();
        *r += 1;
        *r
    }
}

pub fn main() {
    let v = MyStruct::new(1);
    let r = v.declared_as_immutable();

    // MyStruct(RefCell { value: 2 })
    println!("{:?}", v);
}
