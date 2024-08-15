pub fn main() {
    {
        let _s = String::from("hello");

        take_ownership(_s);

        // _s более недоступна, т.к. владение
        // было отдано ф-и

        // borrow of moved value: `_s`
        // println!("{}", _s);
    }

    {
        let _s1 = String::from("hello");

        let (_length, _s2) = take_ownership_and_gives_back(_s1);

        // _s1 более недоступна,
        // владение было отдано функции,
        // а функция отдала владение в _s2
        // println!("{}", _s1);
    }
}

fn take_ownership(_s: String) {}

fn take_ownership_and_gives_back(s: String) -> (usize, String) {
    (s.len(), s)
}