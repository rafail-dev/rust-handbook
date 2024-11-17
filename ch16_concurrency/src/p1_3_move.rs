#![allow(dead_code, unused)]

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword: `move
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // borrow of moved value: `v`
    // println!("Here's a vector: {:?}", v);

    handle.join().unwrap();
}
