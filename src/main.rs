use std::io;
fn main() {
    println!("Guess the number ");
    println!("Enter the number");

    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    println!("You guessed: {str}");
}
