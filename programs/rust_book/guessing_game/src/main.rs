use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!\nPlease input your guess:");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        // mutable guess variable that is a string
        let mut guess: String = String::new();

        // read a line using stdio library, takes a specified mutable reference to
        // a string as input
        // returns a Result type, kind of enum, about how it went
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert string to int, trim before hand
        // expect in case of failure, shadow the previous variable
        // let guess: u32 = guess.trim().parse()
        //    .expect("Input is not a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // switch statement on the Result type of cmp
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
