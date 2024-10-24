#![allow(dead_code, unused)]
fn main() {
    {
        fn add_one_v1(x: u32) -> u32 {
            x + 1
        }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };

        // infer
        let add_one_v3 = |x| x + 1;
        add_one_v3(1);
    }

    {
        let x = Option::Some(1);

        // "|| 0" - closure
        let v = x.unwrap_or_else(|| 0);
    }

    {
        let x = Option::Some(1);

        let mut y = 0;
        // capture
        let closure = || y;
        // cannot assign to `y` because it is borrowed
        // y = 1;
        let v = x.unwrap_or_else(closure);
    }
}
