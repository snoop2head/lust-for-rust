// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

// bringinng library for input/output functionality
use std::io;

// entrypoint
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    
    // storing user input as mutable variable
    let mut guess = String::new(); // String type, associated with new() function

    io::stdin()
        .read_line(&mut guess) // handle input from the user with read_line(), access to mutable variable guess
        .expect("Failed to read line"); // handling potential failure

    println!("You guessed: {}", guess);
}
