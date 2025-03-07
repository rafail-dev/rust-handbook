#![allow(dead_code, unused)]

#[derive(Debug)]
struct A {
    a: u64,
    b: u64,
}

impl A {
    // shortcut for self: &Self
    // заимствует элемент, borrowing
    fn f1(&self) -> u64 {
        self.a + self.b
    }

    // возвращает пустой кортеж (юнит тип)
    // аналог void
    // но мог бы вернуть любой другой тип
    fn f2(&mut self) -> () {
        self.a = 10;
    }

    // поглощает элемент - consuming, ownership
    fn f3(self) -> u64 {
        self.a + self.b
    }
}

fn main() {
    let mut a = A { a: 1, b: 1 };

    // f1 только читает структуру
    let r = a.f1();
    println!("r is {}", r);
    println!("a1 is {:#?}", a);
    println!();

    // f2 мутирует структуру
    a.f2();
    println!("r is {}", r);
    println!("a1 is {:#?}", a);
    println!();

    // f3 забирает во владение структуру
    let r = a.f3();
    println!("r is {}", r);
    // value borrowed here after move
    // println!("{:#?}", a1)
}
