mod submodule_1 {
    pub use submodule_1_1::f;

    mod submodule_1_1 {
        pub fn f() {}
    }
}

pub fn main() {
    // но f определена в submodule_1_1
    submodule_1::f()
}
