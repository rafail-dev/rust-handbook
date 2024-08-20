#[allow(dead_code)]
pub fn main() {
    let _ = vec![1, 2, 3];

    let mut v = vec![1, 2, 3];
    v.push(4);
    let _ = v.pop();
    // [1, 2, 3]
    // println!("{:?}", v);

    // небезопасно,
    // в случае отсутствия значение -
    // thread 'main' panicked at src/p1_vec.rs:9:20:
    let _ = &v[2];

    let third = v.get(2);
    match third {
        Some(_) => {}
        None => {}
    }

    // нельзя иметь одновременно
    // мутабельную и иммутабельную ссылки
    #[allow(unused_variables)]
    let first = &v[0];
    // cannot borrow `v` as mutable because it is also borrowed as immutable
    v.push(6);
    // println!("The first element is: {first}");
    // после мутации (push(6)) данные векторы могут быть
    // перенесены, чтобы оставаться последовательными
}
