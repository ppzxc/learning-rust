use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess The Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please Input Your Guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!!");
                break
            },
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
