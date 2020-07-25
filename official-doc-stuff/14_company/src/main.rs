use std::io;
use std::collections::HashMap;
fn main() {
    println!("Welcome to company!");
    let mut database: HashMap<String,String> = HashMap::new();
    loop {
        println!("Please select ");
        println!("[1] Add employee to database");
        println!("[2] Fetch employees in a department");
        println!("[3] Fetch employees by department");
        let mut text_interface_input = String::new();
        io::stdin().read_line(&mut text_interface_input).expect("Failed to read line");

        match text_interface_input.trim().parse() {
            Ok(num) => {
                match num {
                    1 => add(& mut database),
                    (_) => continue
                }
            },
            Err(c) => {
                println!("{} is an invalid input. Only (1-3) allowed",c);
                continue
            }
        };
    }
}

fn add(database: &mut HashMap<String,String>) {
    let mut input = String::new();
    println!("**************************Adding***************************");
    println!("Enter an employee in the format : ");
    println!("Add Name to Department");
    io::stdin().read_line(&mut input).expect("Failed to read line");


}