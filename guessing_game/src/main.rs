// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

// bringinng library for input/output functionality
use std::io;
use rand::Rng;

// entrypoint
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");
    
    // storing user input as mutable variable
    let mut guess = String::new(); // String type, associated with new() function

    io::stdin()
        .read_line(&mut guess) // handle input from the user with read_line(), access to mutable variable guess
        .expect("Failed to read line"); // handling potential failure

    println!("You guessed: {}", guess);
}
