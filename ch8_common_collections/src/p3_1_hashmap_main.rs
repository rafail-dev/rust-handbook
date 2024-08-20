use std::collections::HashMap;

#[allow(unused_variables, dead_code)]
pub fn main() {
    // https://en.wikipedia.org/wiki/SipHash
    // функцию хеширования можно изменять

    let mut map = HashMap::new();
    map.insert(String::from("a"), 100);
    // значение перезапишется
    map.insert(String::from("a"), 200);

    let map = HashMap::from([
        (String::from("a"), 100),
        (String::from("b"), 50),
        (String::from("c"), 300),
    ]);

    // get - Option<&i32>
    //
    // если ссылка &i32 есть,
    // то копирование значения по ней из heap в stack,
    // иначе передаче None далее
    // т.е. Option<&i32> в Option<i32>
    //
    // unwrap_or( .. ) -  извлечение из Option
    let value_by_a = map.get(&String::from("a")).copied().unwrap_or(0);

    // доступ осуществляется в прозвольном порядке
    for (key, value) in &map {
        println!("{key}: {value}");
    }
}
