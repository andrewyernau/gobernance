fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with(5);

    let y = {
        let x = 3;
        x + 1 // No hay ";" ya que sino se convierte en una sentencia y no devuelve un valor
    };

    println!("The value of y is: {y}");
    println!("five() returns {}", five());
}

fn another_function() {
    println!("Another function");
}

fn another_function_with(x: i32) {
    println!("Another function with param: {}", x);
}

fn five() -> i32 {
    5
}
