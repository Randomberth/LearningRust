use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        println!("");
        println!("You guessed: {}", guess);
        println!("");
        println!("You guessed: {}", secret_number);
            
}
