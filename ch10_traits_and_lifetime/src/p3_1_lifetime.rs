#![allow(unused, dead_code)]

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn main() {
    {
        let string1 = String::from("abcd");

        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        // `string2` does not live long enough
        // println!("The longest string is {}", result);
    }

    {
        let string1 = String::from("short");
        let result;
        {
            let string2 = String::from("long string is long");
            result = first(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {}", result);
    }
}
