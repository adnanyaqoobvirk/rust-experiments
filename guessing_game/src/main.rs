extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let guessable = rand::thread_rng().gen_range(1, 101);

    loop{
        let mut guess = String::new();
        
        println!("Enter your guess: ");
        io::stdin().read_line(&mut guess).expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
        };

        match guess.cmp(&guessable) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}