use std::collections::HashMap;

#[allow(unused_variables, dead_code)]
fn main() {
    let mut map = HashMap::from([
        (String::from("a"), 100),
        (String::from("b"), 50),
        (String::from("c"), 300),
    ]);

    // вставка если значения не было
    map.entry(String::from("d")).or_insert(100);
    // не произведет эффекта
    map.entry(String::from("d")).or_insert(150);

    let value = map.entry(String::from('a')).or_insert(0);
    // * - разыменование
    *value += 1;

    for (key, value) in &map {
        println!("{key}: {value}");
    }
}
