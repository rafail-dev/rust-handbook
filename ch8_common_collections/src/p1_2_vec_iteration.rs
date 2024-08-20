#[allow(dead_code)]
pub fn main() {
    // внутри переборы нельзя изменять/удалять
    // значения вектора
    let v = vec![100, 32, 57];
    #[allow(unused_variables)]
    for i in &v {}

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // * - разыменование для получения
        // значения по ссылке в переменной i
        *i += 50;
    }
}
