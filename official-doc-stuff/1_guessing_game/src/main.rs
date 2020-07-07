use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Guessing game");
    let secret_num = rand::thread_rng().gen_range(1,101);

    loop {
        let mut guess = String::new();
        println!("Enter your number");
        io::stdin().read_line(&mut guess).expect("Could not read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {}",guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Correct");
                break;
            },
        }

    }

}