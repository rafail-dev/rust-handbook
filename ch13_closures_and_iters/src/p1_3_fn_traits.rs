#![allow(unused, dead_code)]
// FnOnce   1 time,
// FnMut    n times, dont move, can mutate captured variables
// Fn       n times, dont move

fn f1<F, T>(f: F) -> ()
where
    F: FnOnce() -> T,
{
    f();
    // use of moved value: `f`
    // f();
}

fn f2<F, T>(mut f: F) -> ()
where
    F: FnMut() -> T,
{
    f();
    f();
}

fn f3<F, T>(f: F) -> ()
where
    F: Fn() -> T,
{
    f();
    f();
}

fn main() {
    {
        let list = vec![1, 2, 3];
        f1(|| println!("{:?}", list));
    }

    {
        let list = vec![1, 2, 3];
        f1(|| list);
        // use of moved value: `list`
        // let v = list;
    }

    {
        let mut list = vec![1, 2, 3];
        f1(|| list.push(4));
        f1(|| list.push(5));
    }

    //

    {
        let list = vec![1, 2, 3];
        f2(&mut || println!("{:?}", list));
    }

    {
        let mut list = vec![1, 2, 3];
        f2(&mut || list.push(4));
        f2(&mut || list.push(5));
    }

    {
        let list = vec![1, 2, 3];
        // cannot move out of `list`, a captured variable in an `FnMut` closure
        // f2(|| list);
    }

    //

    {
        let list = vec![1, 2, 3];
        f3(|| println!("{:?}", list));
    }

    {
        let mut list = vec![1, 2, 3];
        // cannot borrow `list` as mutable, as it is a captured variable in a `Fn` closure
        // f3(|| list.push(4));
    }

    {
        let list = vec![1, 2, 3];
        // cannot move out of `list`, a captured variable in an `Fn` closure
        // f3(|| list);
    }
}
