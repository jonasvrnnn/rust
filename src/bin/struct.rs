struct Person {
    name: String,
    age: i32,
    last_name: String,
}
impl Person {
    fn new(name: String, last_name: String, age: i32) -> Person {
        new_from_input(&name, &last_name);
        Person {
            name,
            last_name,
            age,
        }
    }

    fn say_hello(&self) {
        println!(
            "hello {} {}. You are {} years old.",
            self.name, self.last_name, self.age
        )
    }
}
fn main() {
    println!("Geef je naam");
    let name: String = readline();
    println!("Geef je achternaam");
    let last_name: String = readline();
    println!("Geef je leeftijd");
    let age: i32 = readline().parse().unwrap();
    let person: Person = Person::new(name, last_name, age);
    person.say_hello();
}

fn readline() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("wrong input error.");
    let trimmed_input = input.trim().to_string();
    trimmed_input
}
fn new_from_input(name: &String, last_name: &String) {
    println!("{} {}", name, last_name)
}
