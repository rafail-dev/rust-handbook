// пути
// absolute: crate или ничего для корневого модуля
// relative: self и super или ничего

mod submodule1 {
    pub fn f() {}
}

pub fn main() {
    // relative
    // вызов относительно текушего модуля
    submodule1::f();
    self::submodule1::f();
    super::p3_1_path::submodule1::f();

    // absolute
    // вызов f из главного модуля
    crate::f();
    crate::p3_1_path::submodule1::f();
}
