use std::{cmp::Ordering, io}; // require io to accept input
use rand::Rng; // Rng is a trait

fn main() {
    // generate super secret number from range 1 to 100 and 100 inclusive, 1..=100 is a range expression
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    // mutable guess variable of type string and is empty string
    let mut guess = String::new();

    // why reference ? to easily access variables without copying its content in memory to multiple times
    io::stdin()
        .read_line(&mut guess) // pass reference to read_line to put whatever user passes into guess
        .expect("Failed to read line");

    let guess:u32 = guess.trim().parse().expect("Pick a number between 1 and 100...");

    println!("you guessed {guess}"); // print guessed number
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("Correct!") 
    }
}
