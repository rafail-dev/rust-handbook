#[allow(unused_variables, dead_code)]
pub fn main() {
    let s1 = "hello".to_string();
    let s2 = ", world!".to_string();
    // fn add(self, s: &str) -> String
    let result = s1 + &s2;
    // s1 более недоступна, a
    // s2 доступна
    //
    // + &s3 + &s4
    // "добавочных" sn может быть много

    // при этом другие варианты вызовут ошибку
    // consider borrowing here: `&`
    // let result = s1 + &s2;

    // cannot add `&String` to `&String`
    // let result = &s1 + &s2;

    // s2 могла бы быт мутабельной,
    // но ее мутация после конкатенации
    // не повлияла бы на result

    let s1 = "hello".to_string();
    let s2 = ", world!".to_string();

    let result = format!("{s1} {s2}");
    let result = format!("{} {}", s1, s2);
    // и s1, и s2 доступны
}
