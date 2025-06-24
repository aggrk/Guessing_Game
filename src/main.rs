use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("This is a Guessing game!");

    let random_number = rand::rng().random_range(1..=100);
    println!("The random number is: {}",random_number); 

    println!("Please enter your guess");

    let mut guess = String::new();
    
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read!");
    
    let guess: u32 = guess.trim().parse().expect("Could not convert!");

    println!("You guessed: {}",guess);

    match guess.cmp(&random_number){
        Ordering::Less => println!("The guess is too low!"),
        Ordering::Greater => println!("The quess is too High!"),
        Ordering::Equal => println!("Your guess was Correct!"),
    }
}
