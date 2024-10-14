#![allow(unused, dead_code)]

// instance of ImportantExcerpt can’t outlive the reference it holds in its part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

struct MyStruct2<'a> {
    part1: &'a str,
    part2: &'a str,
}

// ???
// struct MyStruct3<'a, 'b> {
//     part1: &'a str,
//     part2: &'b str,
// }

fn main() {
    {
        let i: ImportantExcerpt<'_>;
        {
            let novel = String::from("Hello. Hello");
            let first_sentence = novel.split('.').next().unwrap();
            i = ImportantExcerpt {
                part: first_sentence,
            };
            print!("{}", i.part);
        }

        // `novel` does not live long enough
        // let later = i;}
    }

    {
        let i: MyStruct2;
        let s1 = String::from("Hello");
        {
            let s2 = String::from("Hello");
            let slice1 = &s1[..];
            let slice2 = &s2[..];

            i = MyStruct2 {
                part1: slice1,
                part2: slice2,
            };
        }
        // `s1` - ok
        // `s2` does not live long enough
        // println!("{}", i.part1);
    }
}
