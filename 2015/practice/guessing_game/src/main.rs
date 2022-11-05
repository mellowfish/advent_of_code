use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut raw_guess = String::new();

        io::stdin()
            .read_line(&mut raw_guess)
            .expect("Failed to read line");

        let trimmed_guess = raw_guess.trim();

        let guessed_number: u32 = match trimmed_guess.parse() {
            Ok(num) => num,
            Err(_) => {
                match trimmed_guess {
                    "quit" => {
                        println!("Ok, bye!");
                        break;
                    },
                    _ => {
                        println!("Only enter a number between 1 and 100...");
                        continue;
                    },
                };
            },
        };

        println!("You guessed: {trimmed_guess}");

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("You guessed too low!"),
            Ordering::Greater => println!("You guessed too high!"),
            Ordering::Equal => {
                println!("You were correct!");
                break;
            },
        }
    }
}
