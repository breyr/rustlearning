struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // Structs are similar to tuples, but with named attributes
    // creating an instance of a struct using key: value pairs
    let mut user1 = User {
        active: true,
        username: String::from("breyr"),
        email: String::from("example@e.com"),
        sign_in_count: 1,
    };

    // to get specific value from struct, use dot notation
    // an entire instance must be mutable, not specific fields
    user1.email = String::from("anotheremail@e.com");

    // creating instances from other instances
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // creating an instance using struct update syntax
    // struct update syntax move the data, here user2
    // cannot be used after user3 is created because in user2, we copy
    // the user1.username into user2
    let user3 = User {
        email: String::from("e@i.com"),
        ..user2
    };

    // Tuple Structs - structs without named fields
    // i.e. struct Color(i32, i32, i32)
    // useful when you want to give the tuple a name
    let white = Color(255, 255, 255);

    // Unit-Like structs
    // structs with no fields, behave similarly to (), the unit type
    // can be useful when you need to implement a trait on some type,
    // but have no data
    let subject = AlwaysEqual;

    // Structs can have references to data that it doesn't own, but this requires
    // lifetimes
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // username: username -> username
        // field init shorthand syntax, no repetition of key and value
        email,
        sign_in_count: 1,
    }
}
