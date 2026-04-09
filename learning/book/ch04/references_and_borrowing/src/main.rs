fn main() {
    let s1 = String::from("hola");

    let len = calcular_longitud(&s1);

    println!("La longitud de '{s1}' es {len}.");

    // referencias mutables
    let mut s = String::from("hola");

    modificar(&mut s);
    println!("String modificado: {s}");
}

fn calcular_longitud(s: &String) -> usize { // es una referencia a un String
    s.len()
} // Aquí, s sale de ámbito. Pero como no tiene el ownership/la propiedad sino
  // que s es solo un prestamo, no se destruye, se regresa al propietario, s1.

fn modificar(un_string: &mut String) {
    un_string.push_str(", mundo!");
}
