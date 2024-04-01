use rand::Rng;
use std::io;
use std::cmp::Ordering;
use std::process;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input.to_lowercase() == "quit" {
            process::exit(0);
        }
    
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}