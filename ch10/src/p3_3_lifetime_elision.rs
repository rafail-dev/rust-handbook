#![allow(unused, dead_code)]
// The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
// In other words, a function with one parameter gets one lifetime parameter:
// fn foo<'a>(x: &'a i32);

// The second rule is that,
// if there is exactly one input lifetime parameter,
// that lifetime is assigned to all output lifetime parameters:
// fn foo<'a>(x: &'a i32) -> &'a i32.

fn foo1_implicitly(s: &str) -> &str {
    &s[..]
}

fn foo1_explicitly<'a>(s: &'a str) -> &'a str {
    &s[..]
}

// a function with two parameters gets two separate lifetime parameters:
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

// error
// missing lifetime specifier
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `s2
// fn foo2_implicitly(s1: &str, s2: &str) -> &str {
//     &s1[..]
// }

fn foo2_explicitly<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
    &s1[..]
}

// ========================================================================

// The third rule is that, if there are multiple input lifetime parameters,
// but one of them is &self or &mut self because this is a method,
// the lifetime of self is assigned to all output lifetime parameters.

struct MyStruct<'a> {
    value: &'a str,
}

impl<'a> MyStruct<'a> {
    fn implicitly(&self) -> &str {
        self.value
    }

    fn explicitly(self: &'a Self) -> &'a str {
        self.value
    }

    // error
    // lifetime may not live long enough
    // fn two_params(&self, s: &str) -> &str {
    //     s
    // }

    fn two_params1(&self, s: &'a str) -> &str {
        s
    }

    fn two_params2<'b>(&self, s: &'b str) -> &'b str {
        s
    }
}

fn main() {
    let s1 = String::from("hello");
    let r1;
    let r2;
    {
        let s = String::from("hello");

        let my_struct = MyStruct { value: s.as_str() };

        r1 = my_struct.two_params1(&s1);
        r2 = my_struct.two_params2(&s1);
    }

    // `my_struct` does not live long enough
    // println!("{}", r1);

    println!("{}", r2);
}
