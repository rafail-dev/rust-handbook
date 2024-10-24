#![allow(dead_code, unused)]

// from standard library
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    // ...
}

fn main() {
    {
        let list = vec![1, 2, 3];

        // lazy evaluation
        let iter = list.iter();

        // loop took ownership of iterator
        for v in iter {
            println!("Got: {}", v);
        }

        // !
        let iter = list.iter();

        iter.for_each(|v| println!("Got: {}", v));
    }

    {
        let list = vec![1, 2, 3];

        // mut!
        let mut iter = list.iter();
        let v1 = iter.next();
        let v2 = iter.next();
        let v3 = iter.next();
        let v4 = iter.next();
        let v5 = iter.next();

        // Some(1), Some(2), Some(3), None, None
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", v1, v2, v3, v4, v5);
    }

    {
        // Consume the Iterator
        let list = vec![1, 2, 3];

        let iter = list.iter();

        let s: i32 = iter.sum();

        // borrow of moved value: `iter`
        // let s2 = iter.sum()
    }

    {
        // Produce Other Iterators
        let list: Vec<i32> = vec![1, 2, 3];

        let iter1 = list.iter();

        let iter2 = iter1.map(|x| x + 1);

        let incremented = iter2.collect::<Vec<i32>>();
        // or
        // let incremented: Vec<i32> = iter2.collect();
    }
}
