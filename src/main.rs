use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = receive_input();

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}

fn receive_input() -> String {
    print!("Input: ");
    io::stdout().flush().expect("flush failed!");

    // variables are immutable by default, add mut to make a variable mutable
    let mut guess = String::new();

    io::stdin()
        // & indicates that this argument is reference
        // references are immutable by default, add &mut to make it mutable
        .read_line(&mut guess)
        .expect("failed to read line");

    return guess;
}
