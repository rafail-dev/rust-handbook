#![allow(dead_code, unused)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};

pub fn main() {
    let v = Rc::new(RefCell::new(5));

    let shared = Rc::new(Cons(Rc::clone(&v), Rc::new(Nil)));

    let list1 = Cons(Rc::new(RefCell::new(3)), Rc::clone(&shared));
    let list2 = Cons(Rc::new(RefCell::new(4)), Rc::clone(&shared));

    println!("before mutate v");
    println!("shared is {:?}", shared);
    println!("list 1 is {:?}", list1);
    println!("list 2 is {:?}", list2);

    *v.borrow_mut() += 10;
    println!("after mutate v");

    println!("shared is {:?}", shared);
    println!("list 1 is {:?}", list1);
    println!("list 2 is {:?}", list2);
}
