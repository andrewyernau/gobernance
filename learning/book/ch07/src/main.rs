use ch07::eat_at_restaurant;

fn main() {
    for event in eat_at_restaurant() {
        println!("{event}");
    }
}
