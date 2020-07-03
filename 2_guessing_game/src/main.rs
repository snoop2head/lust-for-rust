// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

// bringinng library for input/output functionality
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// entrypoint
fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Guess the number!");
    println!("Please input your guess.");
    
    // storing user input as mutable variable
    let mut guess = String::new(); // String type, associated with new() function

    io::stdin()
        .read_line(&mut guess) // handle input from the user with read_line(), access to mutable variable guess
        .expect("Failed to read line"); // handling potential failure

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You got it right!"),
    }
}
