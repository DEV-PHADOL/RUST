use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Hello, world!");
    let secret_number = rand::rng().random_range(1..=100);

    println!("Generated Secret Number : {secret_number}");
    loop {
        println!("Please Enter your Guess : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input.");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid input.");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal =>{
                println!("You Won!");
                break;
            }, 
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small"),
        }
    }
}