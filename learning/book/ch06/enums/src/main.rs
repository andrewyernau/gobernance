#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("El mensaje pide salir."),
            Message::Move { x, y } => println!("Mover hasta ({x}, {y})."),
            Message::Write(text) => println!("Texto: {text}"),
            Message::ChangeColor(r, g, b) => {
                println!("Cambiar color a rgb({r}, {g}, {b})");
            }
        }
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(&home);
    route(&loopback);

    let quit = Message::Quit;
    quit.call();

    let movement = Message::Move { x: 12, y: 24 };
    movement.call();

    let m = Message::Write(String::from("hello!"));
    m.call();

    let background = Message::ChangeColor(30, 30, 40);
    background.call();

    let maybe_selected_block: Option<u8> = Some(4);
    println!("Bloque seleccionado: {:?}", maybe_selected_block);
}

fn route(ip_kind: &IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4(a, b, c, d) => println!("IPv4: {a}.{b}.{c}.{d}"),
        IpAddrKind::V6(address) => println!("IPv6: {address}"),
    }
}
