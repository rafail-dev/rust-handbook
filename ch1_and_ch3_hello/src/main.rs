fn main() {
    let x1 = 5;
    println!("The value of x1 is {}", x1);
    // x1 = 6; // cannot assign twice to immutable variable

    let mut x2 = 5;
    println!("The value of x2 is {x2}");
    x2 = 6;
    println!("The value of x2 is {x2}");

    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    //

    let x3 = 5;

    let x3 = x3 + 1;

    {
        let x3 = x3 * 2;
        println!("The value of x3 in the inner scope is: {x3}");
    }

    println!("The value of x3 is: {x3}");

    //
    // let guess: i32 = "42".parse().expect("Not a number");

    let r = -10 % 3;
    println!("{r}");

    let tup = (500, 6.4, 1);

    let (_, _, z) = tup;
    let x = tup.0;
    println!("The value of x is: {x}");
    println!("The value of z is: {z}");

    //

    let _a = [1, 2, 3, 4, 5];

    another_function(4);

    let x = plus_one(1);
    println!("{x}");

    let mut x = 1;
    let r = loop {
        println!("x is {x}");
        x = x * 2;

        if x > 10_000_000 {
            break x;
        }
    };

    println!("Loop - {r}");

    for number in (1..4).rev() {
        println!("{number}!");
    }

}

fn another_function(x1: i32) {
    let y = {
        let x2 = 3;
        x2 + x1
    };

    println!("Another function {y}")
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
