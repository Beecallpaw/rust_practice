use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please type your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not allowed, type numbers only.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("That's too small, Try again."),
            Ordering::Greater => println!("That's too big, Try again."),
            Ordering::Equal => {
                println!("Congratulations your guess is correct. You win $$$$");
                break;
            }
        }
    }
}
