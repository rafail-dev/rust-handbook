mod submodule_1 {
    pub mod submodule_1_1 {
        pub fn f() {}
    }
}

// пвсевдонимы только для текущей области
// указание модуля перед вызовом функции
// является идиоматичным способом
use submodule_1::submodule_1_1;
use submodule_1::submodule_1_1::f;

// но для подключения структур, перечислений и др.
// идиоматично использовать полный путь
#[allow(unused_imports)]
use std::collections::HashMap;
// но при этом, чтобы различать
// разные Result
use std::fmt;
use std::io;

#[allow(dead_code)]
fn f1() -> fmt::Result {
    Ok(())
}

#[allow(dead_code)]
fn f2() -> io::Result<()> {
    Ok(())
}

pub fn main() {
    // 1ый способ явно показывает то, что
    // f - это функция из модуля,
    // а не локальная
    submodule_1_1::f();
    f()
}
