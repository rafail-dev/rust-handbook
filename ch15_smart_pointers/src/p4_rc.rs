#![allow(dead_code, unused)]

mod via_box {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use self::List::{Cons, Nil};

    pub fn main() {
        let a = Cons(1, Box::new(Cons(2, Box::new(Nil))));

        let b = Cons(1, Box::new(a));
        // use of moved value: `a`
        // let c = Cons(1, Box::new(a));
    }
}

mod via_rc {
    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use self::List::{Cons, Nil};

    pub fn main() {
        let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));

        // 1
        println!("Reference count of `a`: {}", Rc::strong_count(&a));

        let b = Cons(1, Rc::clone(&a));

        // 2
        println!("Reference count of `a`: {}", Rc::strong_count(&a));

        {
            let c = Cons(1, Rc::clone(&a));

            // 3
            println!("Reference count of `a`: {}", Rc::strong_count(&a));
        }

        // 2
        println!("Reference count of `a`: {}", Rc::strong_count(&a));
    }
}

pub fn main() {
    via_box::main();
    via_rc::main();
}
