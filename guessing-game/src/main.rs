use std::io::stdin;
use rand::distributions::{Distribution, Uniform};

fn main() {
    const MAX_RANGE: u8 = 11;
    let mut rng = rand::thread_rng();
    let rng_range = Uniform::from(1..MAX_RANGE);

    let correct_number = rng_range.sample(&mut rng);
    loop {
        println!("Guess a number betweeb 1 and {MAX_RANGE}");
        println!("Input your guess");

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        if guess.trim() == correct_number.to_string() {
            println!("Correct");
            break;
        } else {
            let guess_as_u8 = u8::from_str_radix(guess.trim(), 10).unwrap();
            let hint = if guess_as_u8 < correct_number {
                "higher"
            } else {
                "lower"
            };
            println!("Guess {}!", hint);
        }
    }
}
