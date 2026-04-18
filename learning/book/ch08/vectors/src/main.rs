fn main() {
    let mut v = vec![1, 2, 3, 4, 5]; // Vector de <i32>
    v.push(6);

    // let does_not_exist = %v[100]; -> PANICS
    let does_not_exist = v.get(100); // We get an Option Some or None
    match does_not_exist {
        Some(does_not_exist) => println!("WOW! It exists!"),
        None => println!("There is no 100th element, obviously"),
    }

    // Podemos iterar sobre el vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // Usamos operador desreferencia '*' para poder cambiar el valor al que se refiere
        // de la referencia mutable
        //
        // Se verá mejor en el capitulo 15
    }
}
