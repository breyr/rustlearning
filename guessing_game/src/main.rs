use rand::Rng; // trait
use std::cmp::Ordering; // enum
use std::io; // enum

fn main() {
    println!("Guess the number!");

    // start..=end (start and end are both inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // mutable reference
            .expect("Failed to read line");

        // parse converts here from string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // return the num value that parse produced
            Err(_) => continue, // _ is a catch all value
        };

        println!("You guessed: {guess}");

        // cmp returns a variant of Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
