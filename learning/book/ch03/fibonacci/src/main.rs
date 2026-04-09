use std::io;

fn main() {
    println!("Enter a fibonacci number times:");

    'main_loop: loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input : u16 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input");
                continue 'main_loop;
            }
        };

        print_fibonacci(input);
    }
}

fn print_fibonacci(x: u16) {
    let mut prev : u128 = 0;
    let mut curr : u128 = 1;

    for i in 0..x {
        if i == 0 {
            println!("0");
            continue;
        }
        if i == 1 {
            println!("1");
            continue;
        }
        let next : u128 = prev + curr;
        println!("{}", next);
        prev = curr;
        curr = next;
    }
}
