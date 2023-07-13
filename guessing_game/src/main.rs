use std::io; // add io from standard library into scope
use rand::Rng; // random number generators
use std::cmp::Ordering; // comparison

fn main() {
    let secret_number = rand::thread_rng()
        .gen_range(1..=100); //
    
   //  println!("secret sauce is currently {}", secret_number);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        // let granny = 5; // immutable by default
        // let kanzi: i32 = 6;
        let mut guess = String::new(); // explicitly made mutable

        io::stdin()
            .read_line(&mut guess) // mutable reference
            .expect("Failed to read line"); // error handling

        let guess: u32 = match guess
            .trim() // remove white space and new line character
            .parse() // converts to another type
            {
                // error handling
                Ok(num) => num,
                Err(_) => continue, // ignores non numbers
            };

        println!("You guessed: {guess}");
        // println!("But apply boiz would be {} for granny and {} if Kanzi", granny, kanzi); // 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit loop
            }
        }

    }
    
}
