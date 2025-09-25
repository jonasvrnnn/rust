fn main() {
    let mut first_name: &str = "Joness";
    greet(first_name);

    first_name = "bob";
    greet(first_name);
}
fn greet(some_name: &str) {
    println!("{some_name}")
}
