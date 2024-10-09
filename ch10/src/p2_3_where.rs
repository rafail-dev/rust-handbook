#![allow(unused, dead_code)]

use std::{
    fmt::{Debug, Display},
    ops::Add,
};

fn foo<T, U>(v1: T, v2: U) -> (U, T)
where
    T: Copy + Add,
    U: Display + Debug,
{
    (v2, v1)
}
