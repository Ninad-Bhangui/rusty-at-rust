use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let v = vec![1, 2, 3];
    // v[99]; //Run with RUST_BACKTRACE=1 for backtrace and panic

    // let f: u32 = File::open("hello.txt");
    println!("{}", read_username_from_file("exists.txt")?); //File exists. Does not panic
    let non_panic_string = match read_username_from_file("donotpanic.txt") {
        //File does not exist. But do not panic and handle it instead
        Ok(s) => s,
        Err(e) => {
            println!("Printing this instead of panicking!");
            String::from("avoided panic")
        }
    };
    println!("{}", read_username_from_file("notexists.txt")?); //File does not exists panic
    Ok(())
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}
