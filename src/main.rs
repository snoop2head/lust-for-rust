// HELLO WORLD
// fn main() {
    // Statements here are executed when the compiled binary is called
//     println!("Hello, world!");
// }


// FERRIS CHARACTER DRAWING
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout  = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width   = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

// 