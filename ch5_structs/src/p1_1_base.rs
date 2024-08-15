struct User {
    _active: bool,
    _username: String,
    email: String,
    _sign_in_count: u64,
}

pub fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let updated_user1 = User {
        _active: true,
        ..user1
    };

    // println!("{}", user1.email);
    // borrow of moved value: `user1.email`
    // move occurs because `user1.email` has type `String`,
    // which does not implement the `Copy` traitrustc

    // user1 более недоступен
    // но user1 был бы достyпен, если бы были заданы новые
    // username и email
    // тк остальные поля реализуют типаж Copy
    // и копируются в стеке

    println!("{}", updated_user1.email);
}

fn build_user(_username: String, email: String) -> User {
    User {
        _active: false,
        _username, // as username: username
        email,
        _sign_in_count: 0,
    }
}
