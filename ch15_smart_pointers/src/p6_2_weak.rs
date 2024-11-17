#![allow(dead_code, unused)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

use self::List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn main() {
    let a = Rc::new(Cons(1, RefCell::new(Weak::new())));

    // // 1
    // println!("a strong_count = {}", Rc::strong_count(&a));
    // // Nil
    // println!("a next item = {:?}", a.tail());
    // println!("---");

    let b = Rc::new(Cons(2, RefCell::new(Rc::downgrade(&a))));

    // // 2
    // println!("a strong_count = {}", Rc::strong_count(&a));
    // // 1
    // println!("b strong_count = {}", Rc::strong_count(&b));
    // // Cons 1
    // println!("b next item = {:?}", b.tail());
    // println!("---");

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }

    // 2
    println!("a strong_count = {}", Rc::strong_count(&a));
    // 2
    println!("b strong_count = {}", Rc::strong_count(&b));

    // after exiting the scope
    // a strong_count = 1
    // b strong_count = 1

    // cycle
    // fatal runtime error: stack overflow
    // println!("a next item = {:?}", a.tail());
}
