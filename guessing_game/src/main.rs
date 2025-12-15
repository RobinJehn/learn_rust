use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number! Please provide your input:");

    let rnd = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input");

        let input = input.trim_end();
        println!("You guessed {input}");

        let num = match input.parse::<i64>() {
            Ok(value) => value,
            Err(err) => {
                println!("error: {err}.\nPlease try again.");
                continue;
            }
        };

        match rnd.cmp(&num) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("More"),
            Ordering::Equal => break,
        }
    }

    println!("Your guess was right!");
}
