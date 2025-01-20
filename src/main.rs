use std::io;

fn main() {
    println!("Gess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guessed: {}", guess);
    // println!("Your guessed: {guess}");
}