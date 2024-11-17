#![allow(dead_code, unused)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn main() {
    let user = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    let updated_user = User {
        active: true,
        ..user
    };

    // println!("{}", user.email);

    // borrow of moved value: `user1.email`
    // move occurs because `user1.email` has type `String`,
    // which does not implement the `Copy` traitrustc

    // user1 более недоступен
    // но user1 был бы достyпен, если бы были заданы новые
    // username и email
    // тк остальные поля реализуют типаж Copy
    // и копируются в стеке

    println!("{}", updated_user.email);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: false,
        username, // as username: username
        email,
        sign_in_count: 0,
    }
}
