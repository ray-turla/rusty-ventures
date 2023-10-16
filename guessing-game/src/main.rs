use std::{
    io::stdin,
    cmp::Ordering
};
use rand::distributions::{Distribution, Uniform};

fn main() {
    const MAX_RANGE: u8 = 10;
    let mut rng = rand::thread_rng();
    let rng_range = Uniform::from(1..=MAX_RANGE);

    let correct_number = rng_range.sample(&mut rng);
    loop {
        println!("Guess a number betweeb 1 and {MAX_RANGE}");
        println!("Input your guess");

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You guessed: {guess}");
                println!("INVALID. Guess must be a number");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&correct_number) {
            Ordering::Less => println!("Guess Higher!"),
            Ordering::Equal => {
                println!("Correct");
                break;
            },
            Ordering::Greater => println!("Guess Lower!")
        };
    }
}
