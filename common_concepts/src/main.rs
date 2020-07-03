// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

fn main() {
    let mut x = 5;
    println!("The value of x is {}",x);
    x = 6;
    println!("Mutated value of x is {}",x);
}
