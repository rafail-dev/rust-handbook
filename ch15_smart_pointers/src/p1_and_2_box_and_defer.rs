#![allow(dead_code, unused)]

use std::ops::Deref;

// recursive type `List` has infinite size
// enum List {
//     Cons(i32, List),
//     Nil,
// }
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// or "type `MyBox<{integer}>` cannot be dereferenced"
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn main() {
    {
        // The Box<T> type is a smart pointer because it implements the Deref trait,
        // which allows Box<T> values to be treated like references.
        let x = 5;
        let copy = Box::new(x);
        assert_eq!(5, *copy);
    }

    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let v = MyBox::new(String::from("Hello"));
        hello(&v);

        // manually dereferencing
        hello(&(*v)[..]);

        // From &T to &U when T: Deref<Target=U>
        // From &mut T to &mut U when T: DerefMut<Target=U>
        // From &mut T to &U when T: Deref<Target=U>
    }
}

fn hello(v: &str) {}
