#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("email@mail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("new_email@mail.com");
    println!("user1 actualizado: {:?}", user1);

    let user2 = User {
        email: String::from("second@mail.com"),
        ..user1
    };
    println!("user2 usando update syntax: {:?}", user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    println!(
        "Color como tuple struct: ({}, {}, {})",
        black.0, black.1, black.2
    );
    println!(
        "Point como tuple struct: ({}, {}, {})",
        origin.0, origin.1, origin.2
    );
    println!("Unit-like struct: {:?}", subject);

    let user3 = build_user(
        String::from("builder@mail.com"),
        String::from("builder_user"),
    );
    println!("user3 creado desde funcion: {:?}", user3);
    println!(
        "Campos de user3 -> activo: {}, username: {}, email: {}, inicios: {}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
