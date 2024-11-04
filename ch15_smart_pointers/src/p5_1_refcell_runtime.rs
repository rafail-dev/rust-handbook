#![allow(dead_code, unused)]
use std::cell::RefCell;

fn main() {
    {
        // runtime panic
        // already borrowed: BorrowMutError
        let v = RefCell::new(1);
        let immut_ref = v.borrow();
        let mut_ref = v.borrow_mut();
        *v.borrow_mut() += 1;
        println!("{}", *immut_ref);
    }
}
