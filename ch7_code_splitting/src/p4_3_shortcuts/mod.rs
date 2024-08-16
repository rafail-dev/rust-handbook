#![allow(unused_imports)]

mod submodule_1 {
    // use std::cmp::Ordering;
    // use std::io;
    // to
    use std::{cmp::Ordering, io};
}

mod submodule_2 {
    // use std::io;
    // use std::io::Write;
    // to

    use std::io::{self, Write};
}

mod submodule_3 {
    // все открытые элементы в текущую область,
    // осторожно
    use std::collections::*;
}

pub fn main() {}
