#![allow(dead_code, unused)]
use std::cell::RefCell;

fn main() {
    // {
    //     let s = String::from("hello");
    //     let immut_ref = &s;
    //     let mut mut_ref = &mut &s; // compile error
    //     println!("{}", immut_ref);
    // }

    {
        // runtime panic
        // already borrowed: BorrowMutError
        let v = RefCell::new(1);
        let immut_ref = v.borrow();
        let mut mut_ref = v.borrow_mut();
        *mut_ref += 1;
        println!("{}", *immut_ref);
    }

    {
        // runtime panic
        // already borrowed: BorrowMutError
        let v = RefCell::new(1);
        let immut_ref = v.borrow();
        println!("{}", *immut_ref);

        let mut mut_ref = v.borrow_mut();
        *mut_ref += 1;
        println!("{}", *mut_ref);
    }

    {
        let v = RefCell::new(1);
        {
            let immut_ref = v.borrow();
            println!("{}", *immut_ref);
        }

        let mut mut_ref = v.borrow_mut();
        *mut_ref += 1;
        println!("{}", *mut_ref);
    }

    {
        let v = RefCell::new(1);
        let immut_ref = v.borrow();
        println!("{}", *immut_ref);
        drop(immut_ref);

        let mut mut_ref = v.borrow_mut();
        *mut_ref += 1;
        println!("{}", mut_ref);
    }
}
