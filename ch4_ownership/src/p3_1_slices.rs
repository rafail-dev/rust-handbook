pub fn main() {
    {
        let mut s = String::from("hello world");

        let _first_word_ends_at = _first_word(&s);

        s.clear();
        // очевидно, что тут first_word_ends_at
        // уже неактуально, но нет способа узнать об этом
    }

    {
        let s = String::from("hello world");

        let _hello = &s[0..5];

        // String хранит
        // - name
        // - ptr на heap
        // - len
        // - capacity

        // &str хранит
        // - name
        // - ptr на heap
        // - len
    }

    {
        let mut s = String::from("hello world");

        let _hello = _first_word_improved(&s);

        // нельзя мутировать строку пока есть иммутабельная ссылка
        // cannot borrow `s` as mutable because it is also borrowed as immutable
        // s.clear();
        // println!("{}", _hello);
        s.clear();
    }

    {
        let mut s = String::from("hello world");
        s.push('!');
        // s = String::from("another");
        println!("{}", s)
    }

    {
        let mut s = String::from("hello world");

        let _hello = &mut s[0..5];
        // доступен ряд мутирующих методом, которые не изменяют длину
        _hello.make_ascii_uppercase();
        // no method named `push` found for mutable reference `&mut str` in the current scope
        // _hello.push('1');

        // HELLO world
        println!("{}", s)
    }

    {
        let mut _s1: String = String::from("hello world");

        let mut _hello = &mut _s1[0..5];

        _hello.make_ascii_uppercase();
    }

}

fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// &String и &str

fn _first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
