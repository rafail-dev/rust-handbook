#[allow(dead_code)]

// объявление подмодулей
// - в этом же файле
// ./module_name.rs
// ./module_name/mod.rs
mod submodule_1 {
    pub fn f_public() {}

    // по умолчанию ф-ции приватны
    // видны в дочерних модулях
    fn f_private() {}

    // дочерние модули также
    // по умолчанию приватные
    pub mod submodule_11 {
        pub fn f() {}
    }
}

mod submodule_2;

mod submodule_3;

// use для созданию псевдонимов
use submodule_1::submodule_11;

pub fn main() {
    submodule_1::f_public();
    submodule_2::f();
    submodule_3::f();

    // function `f_private` is private
    // submodule_1::f_private();

    submodule_11::f();
}
