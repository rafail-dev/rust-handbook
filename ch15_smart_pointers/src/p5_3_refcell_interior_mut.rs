#![allow(dead_code, unused)]

use std::cell::RefCell;

#[derive(Debug)]
struct MyStruct(RefCell<u8>);

#[derive(Debug)]
struct MyStruct2 {
    x: RefCell<u8>,
}

trait MyTrait {
    fn declared_as_immutable(&self) -> ();
}

impl MyStruct {
    fn new(v: u8) -> MyStruct {
        MyStruct(RefCell::new(v))
    }
}

impl MyTrait for MyStruct {
    fn declared_as_immutable(&self) -> () {
        let mut r = self.0.borrow_mut();
        *r += 1;
    }
}

impl MyStruct2 {
    fn new(x: u8) -> MyStruct2 {
        MyStruct2 { x: RefCell::new(x) }
    }
}

impl MyTrait for MyStruct2 {
    fn declared_as_immutable(&self) -> () {
        *self.x.borrow_mut() += 4;
    }
}

trait MyTrait2<T> {
    fn f(v: &T) -> u8;
}

pub fn main() {
    let v = MyStruct2::new(1);
    let r = v.declared_as_immutable();

    // MyStruct(RefCell { value: 2 })
    println!("{:?}", v);
}
