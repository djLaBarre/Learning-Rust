use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut i: i32 = 10;

    while i > 0 {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".truecolor(23, 3, 252)),
            Ordering::Greater => println!("{}", "Too big!".truecolor(3, 194, 252)),
            Ordering::Equal => { println!("{}", "You Win!".green());
                break;
            }
        }
        i = i - 1;
    }
    if i == 0 {
    println!("{}", "You lose! Try again!".red());
    }
}
