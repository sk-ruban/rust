// entire instance must be mutable if set as such, not only select fields

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs - when naming each field is redundant
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        // .. specifies that the remaining fields should have the same value as user1
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // not necessary to have it as username: username
        username,
        email,
        sign_in_count: 1,
    }
}