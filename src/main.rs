use std::cmp::Ordering;

use rand::Rng;
use std::io;

fn main() {
    let mut guess = String::new();
    let secret_number = rand::rng().random_range(1..=100);

    println!("Guess the number");
    println!("Please input your guess");

    io::stdin().read_line(&mut guess);
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!, the secret number is {secret_number}"),
        Ordering::Greater => println!("Too big!, the secret number is {secret_number}"),
        Ordering::Equal => println!("You win!"),
    }
}
