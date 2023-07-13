use std::io; // add io from standard library into scope

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let granny = 5; // immutable by default
    let kanzi: i32 = 6;
    let mut guess = String::new(); // explicitly made mutable

    io::stdin()
        .read_line(&mut guess) // mutable reference
        .expect("Failed to read line"); // error handling

    println!("You guessed: {guess}");
    println!("But apply boiz would be {} for granny and {} if Kanzi", granny, kanzi); // 
}
