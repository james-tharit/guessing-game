use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("The secret number is: {secret_number}");

    loop {

        println!("please input your guess."); 
    
        // variables are immutable by default, add mut to make a variable mutable
        let mut guess = String::new();

        io::stdin()
            // & indicates that this argument is reference
            // references are immutable by default, add &mut to make it mutable
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse().expect("please type a number!");

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
