#![allow(unused, dead_code)]

trait MyTrait {
    fn my_f(&self) -> i32;
}

struct My1 {
    x: i32,
    y: i32,
}

struct My2 {
    x: i32,
    y: i32,
}

impl MyTrait for My1 {
    fn my_f(&self) -> i32 {
        self.x * self.y
    }
}

impl MyTrait for My2 {
    fn my_f(&self) -> i32 {
        self.x * self.y
    }
}

fn foo1(condition: bool) -> impl MyTrait {
    My1 { x: 10, y: 20 }
}

fn foo2(condition: bool) -> impl MyTrait {
    if condition {
        My1 { x: 10, y: 20 }
    } else {
        My1 { x: 20, y: 30 }

        // error
        // expected `My1`, found `My2`
        // My2 { x: 20, y: 30 }
    }
}
