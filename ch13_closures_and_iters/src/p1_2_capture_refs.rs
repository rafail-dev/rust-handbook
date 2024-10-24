#![allow(dead_code, unused)]

use std::thread;

fn main() {
    // three ways:
    // - borrowing immutably,
    // - borrowing mutably
    // - taking ownership

    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }
    //
    {
        let mut list = vec![1, 2, 3];

        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(4);

        // cannot borrow `list` as immutable because it is also borrowed as mutable
        // println!("After calling closure: {:?}", list);

        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }
    //
    {
        let list = vec![1, 2, 3];
        let moves = move || println!("captured {:?} by value", list);

        // borrow of moved value: `list`
        // println!("{:?}", list)
    }
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }
}
