// crates
// - наименьший объем кода, который рассматривается за раз компилятором
// - содержат modules
// - бывают: binary (c fn main), library (без fn main)
//
// package
// - один или несколько crates
// - 0-n binary crates
// - 0-1 library crate
//
// default
// - src/main.rs
// - src/lib.rs
// custom in Cargo.toml
fn main() {}
