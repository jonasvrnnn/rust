fn main() {
    let mut numbers: Vec<u32> = Vec::new();
    let mut stoppen: bool = false;
    while stoppen == false {
        println!("Geef een nummer, als je wilt stoppen druk dan op enter.");
        let number: String = read_string();
        if number == "" {
            stoppen = true;
        } else {
            numbers.push(number.parse::<u32>().unwrap());
        }
    }
    let gemiddelde: u32 = numbers.iter().copied().fold(1, |a, e| a + e) / (numbers.len() as u32);
    let studenten_minder_dan_10: Vec<u32> = numbers.iter().copied().filter(|&e| e < 10).collect();
    println!("Het gemiddelde is {}", gemiddelde);
    println!(
        "Het aantal studenten minder dan 10: {}",
        studenten_minder_dan_10.len()
    )
}

fn read_string() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error, foute input");
    input.trim().to_string()
}

fn read_number() -> u8 {
    let input: String = read_string();
    input.parse::<u8>().unwrap()
}
