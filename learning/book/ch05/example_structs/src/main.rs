#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;
    println!(
        "Area usando valores separados: {}",
        area_with_separate_values(width, height)
    );

    let rectangle_tuple = (30, 50);
    println!(
        "Area usando tupla: {}",
        area_with_tuple(rectangle_tuple)
    );

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectangulo: {:?}", rectangle);
    println!(
        "Area usando struct: {}",
        area_with_struct(&rectangle)
    );
}

fn area_with_separate_values(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
