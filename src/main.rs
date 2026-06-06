use std::io; // Import the io library

fn main() {
    println!("Guess the number!"); // Print the message to the console
    println!("Please input your guess.");
    let mut guess = String::new(); // Create a new string mutablevariable
    io::stdin()
        .read_line(&mut guess) // Read the line from the console
        .expect("Failed to read line"); // Print the error message if the line is not read
    println!("You guessed: {guess}"); // Print the guess to the console
}
