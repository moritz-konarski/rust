use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please guess a number between 1 and 100.");
    
    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small, guess again!"),
            Ordering::Greater   => println!("Too big, guess again!"),
            Ordering::Equal     => {
                println!("You win! It was {}.", secret_number);
                break;
            }
        }
    }
}
