extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game!");
    println!("A random number is now generated and it's up to you to guess which.");
    
    let secret_number = rand::thread_rng().gen_range(1, 101); // Generate a random number
    
    loop {
        println!("Guess a number:");

        let mut guess = String::new(); // Create a string the user supplied value will go into
        io::stdin().read_line(&mut guess) // Read the user's entered value and put it into the guess variable
        .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() { // Parse the value and shadows it as an usigned 32 bit integer
            Ok(num) => num,
            Err(_) => continue
        }; 

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The entered number is too low!"),
            Ordering::Greater => println!("The entered number is too high!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }   
    }
}
