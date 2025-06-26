use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("This is a Guessing game!");

    let random_number = rand::rng().random_range(1..=100);

    loop{
        println!("Please enter your guess");

    let mut guess = String::new();
    
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read!");
    
    let guess: u32 =match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("The input must be a number, try again!");
            continue;
        }
    };

    println!("You guessed: {}",guess);
    
    match guess.cmp(&random_number){
        Ordering::Less => println!("The guess is too low!"),
        Ordering::Greater => println!("The guess is too High!"),
        Ordering::Equal =>{
            println!("Your guess was Correct, you win!");
            break;
        }
    }
}
}
