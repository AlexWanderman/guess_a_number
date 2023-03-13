use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

fn main() {
    // Game generates a random number from 1 too 100
    println!("Guess the number from 1 to 100!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // Ask for input
        print!("Your guess: ");
        stdout().flush().unwrap();

        // Read the guess
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Could not read a line!");

        // Filter invalid values
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        // Filter values outside the range 1-100
        if guess < 1 || guess > 100 {
            println!("Guess from 1 to 100, not {guess}");
            continue;
        }

        // Check win condition
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
