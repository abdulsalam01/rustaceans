use std::io;

pub fn guess() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // mutable variable using keywords "mut"
    let mut guess = String::new();
    // handle input, pass by reference, safety in memory, 
    // can use &guess, but can't be changed
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}