struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("myemail@example.com"),
        username: String::from("hogehoge"),
        active: true,
        sign_in_count: 1
    };

    let user2 = build_user(String::from("newmail@example.com"), String::from("hugahuga"));
    let user3 = User {
        email: String::from("finalmail@example.com"),
        username: String::from("super"),
        ..user2
    };

    println!("users whose active is {} and sign_in_count is {}", user1.active, user1.sign_in_count);
    println!("user1: {}, {}", user1.username, user1.email);
    println!("user2: {}, {}", user2.username, user2.email);
    println!("user3: {}, {}\n", user3.username, user3.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
