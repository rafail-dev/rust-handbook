#[allow(dead_code)]
mod submodule_1 {
    // перечисления также
    // по умолчанию приватны,
    // но из значения публичны
    pub enum Enum1 {
        V1,
        V2,
    }

    // в структурах также
    // необходимо помечать поля
    // публичными
    pub struct Struct1 {
        pub a: i64,
        pub b: i64,
    }

    // такую структуру нельзя
    // создать извне, т.к.
    // поле b приватное
    pub struct Struct2 {
        pub a: i64,
        b: i64,
    }
    // в этом случае можно
    // определить фабричный метод
    impl Struct2 {
        pub fn new(a: i64) -> Struct2 {
            Struct2 { a, b: 10 }
        }
    }
}

pub fn main() {
    let _ = submodule_1::Enum1::V1;

    let _ = submodule_1::Struct1 { a: 1, b: 1 };

    let v = submodule_1::Struct2::new(1);
    let _ = v.a;
    // field `b` of `Struct2` is private
    // let _ = v.b;
}
