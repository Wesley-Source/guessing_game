#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("============================================");
    println!("Guess the number!");
    println!("============================================");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new();
        println!("Please input you guess");

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}

            Err(error) => {
                println!("Error reading line: {error}");
                break;
            }
        }

        let int_guess: u32;

        match guess.trim().parse::<u32>() {
            Ok(content) => {
                int_guess = content;
            }
            Err(error) => {
                println!("{error}");
                println!("Please input a number.");
                continue;
            }
        }

        match int_guess.cmp(&secret_number) {
            Ordering::Greater => {
                println!("Too high!");
            }
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Less => {
                println!("Too low!");
            }
        }
    }
}
