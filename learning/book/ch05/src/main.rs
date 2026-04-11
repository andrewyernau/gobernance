fn main() {
    let user1 = User {
        active: true,
        username: String::from("Username123"),
        email: String::from("email@mail.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("newemail@mail.com");

    let user2 = User {
        email: String::from("secondmail@mail.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    //EXAMPLE
    let width1 = 30;
    let height1 = 50;
    println!(
        "Area: {}",
        area(width1, height1)
    );
    // No hay "coorrelacion con w y h"
    let rect1 = (30,50);
    println!(
        "Area: {}",
        area2(rect1)
    );
    // Problema: no sabemos que son ancho y alto en la funcion
    // Solucion: refactorizar con structs
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "Area: {}",
        area3(&rect2)
    );
    println!("Rect2: {:?}", rect2);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: (50),
    };
    dbg!(&rect2);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;
