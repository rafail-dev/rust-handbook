#[allow(unused_variables, dead_code)]
pub fn main() {
    // мутирование строки
    let mut s = String::from("hello");
    // hello, world
    s.push_str(", world");
    // hello, world!
    s.push('!');
    // hello, world
    let _ = s.pop();
    // hello
    // опасная оперция, 5 это кол-во байт,
    // но при этом символы могут занимать по n байт,
    // а если символ "обрежется", то panicked at
    // assertion failed: self.is_char_boundary(new_len)
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    s.truncate(5);
    // ""
    s.clear();
}
