#[allow(unused_variables, dead_code)]
pub fn main() {
    {
        let s = String::from("hello");

        // невозможно обращаться по индексу
        // из-за неопределенности "byte или символ?"
        // the type `str` cannot be indexed by `{integer}`
        // let h = &s[0];
    }

    // при этом
    {
        let s1 = String::from("hello");
        let slice1 = &s1[0..3];
        // hel
        println!("{}", slice1);

        let s2 = String::from("привет");

        // thread 'main' panicked at p2_4_string_indexing.rs:20:25:
        // byte index 3 is not a char boundary;
        // it is inside 'р' (bytes 2..4) of `привет`
        // let slice2 = &s2[0..3];
        // println!("{}", slice2);
    }

    // лучше явно указывать, что нужны байты / символы
    {
        let s = String::from("привет");

        for c in s.chars() {
            println!("{}", c);
        }

        // ри
        let substring: String = s.chars().skip(1).take(2).collect();
        let substring = s.chars().skip(1).take(2).collect::<String>();

        // ['р', 'и']
        let substring = s.chars().skip(1).take(2).collect::<Vec<char>>();

        for b in s.bytes() {
            println!("{}", b);
        }
    }
}
