fn main() {
    println!("Hello, world!");

    let u1 = User {
        active: true,
        username: String::from("shepard"),
        email: String::from("eviltomorrow@gmail.com"),
        sign_in_count: 1,
    };

    println!(
        "{}, {}, {}, {}",
        u1.active, u1.username, u1.email, u1.sign_in_count
    );
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
