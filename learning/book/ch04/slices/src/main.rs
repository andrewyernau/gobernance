fn main() {
    let sentence = String::from("hola mundo sandbox");
    let first = first_word(&sentence);
    println!("Primera palabra en String: {first}");

    let literal = "rust book";
    println!("Primera palabra en literal: {}", first_word(literal));

    let values = [10, 20, 30, 40, 50];
    let middle = &values[1..4];
    println!("Slice de array: {:?}", middle);
}

fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[..index];
        }
    }

    text
}
