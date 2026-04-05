fn main() {
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x = {x}");
    } // Ámbito interno

    println!("x = {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {spaces}");

    let guess: u32 = "42".parse().expect("Not a number!"); // Importante "u32" ya que en compilación se verifica el tipo.
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("--ARRAYS--");
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    // We use Arrays when we are sure that the number of elements is fixed.
    // If not, we should use a Vector instead.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
