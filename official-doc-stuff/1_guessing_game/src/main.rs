use rand::Rng; //Rng is a trait
use std::cmp::Ordering;
use std::io;

fn main() {
    //Mutability test (Below will throw `cannot assign twice to immutable variable` when uncommented and compiled)
    /*
        let immutable_var = 5;
        immutable_var = immutable_var + 10;
    */

    println!("Welcome to Guessing game");
    //rand::thread_rng returns a generator for current thread, gen_range generates it between lower(inclusive) and upper(exclusive) bounds.
    let secret_num = rand::thread_rng().gen_range(1, 101);
    //loop keyword is used for infinite loop. Use break/continue

    loop {
        //String::new returns new instance of string
        let mut guess = String::new();
        println!("Enter your number");
        /*read_line returns io::Result
        & - reference
        &mut - mutable reference
        expect is method on instance of io::Result
        Err : crash and display error message
        ok : return value held by Ok
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line");
        /*
        Have to make sure both types are same.
        Below statement is called shadowing. Reuse variable instead of new variable.
        Could have used expect but in this case the loop repeats execution and asks for user input again.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
        }
    }
}
