#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("m@m.com"), String::from("hombre"));

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{:#?}", user2);
    println!(
        "user1 -> active: {}, sign_in_count: {}",
        user1.active,
        user1.sign_in_count,
        // user1.username and user1.email cannot be used because they're Strings. in user2 assignment they got moved
    );
}
