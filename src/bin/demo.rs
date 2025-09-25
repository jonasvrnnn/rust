fn main() {
    println!("");
    let input: String = readline();
    if input.to_lowercase() == "joness" {
        println!("hello {input}")
    } else if input.to_lowercase() == "albert" || input == "maddie" {
        println!("watsupp {input}")
    } else if input == "" {
        println!("type een naam in alsjeblieft!!!")
    }
}
fn readline() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("wrong input");
    let trimmed_input: String = input.trim().to_string();
    trimmed_input
}
