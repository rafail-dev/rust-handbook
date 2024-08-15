#[derive(Debug)]
struct A {
    #[allow(dead_code)]
    a: String,

    #[allow(dead_code)]
    b: u32,
}

pub fn main() {
    let a1 = A {
        a: String::from("str1"),
        b: 1,
    };

    // to stdout
    println!("{:?}", a1); //or println!("{:?}",&a1);
    println!("{a1:#?}");
    // {:?} - Debug
    // {:#?} - pretty-print Debug

    // to stderr
    dbg!(&a1);
    dbg!(&a1);

    // but error
    // dbg!(a1);
    // -------- value moved here
    // dbg!(&a1);

    let a2 = A {
        a: String::from("str1"),
        b: dbg!(1 * 2),
    };

    println!("{:?}", a2);
}
