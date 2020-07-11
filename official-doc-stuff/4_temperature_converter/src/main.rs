use std::io;

fn main() {
    println!("Hello, world!");

    loop {
        println!("Type in the temperature in the format 10F/100C (Fahrehnheit/Celsius). Press q/Q to exit,");

        let temp_string = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read line!");

            input.to_uppercase().trim().to_string()
        };
        if temp_string == "Q" {
            println!("Exiting ..");
            break;
        } else if temp_string.chars().last() != Some('C') && temp_string.chars().last() != Some('F')
        {
            println!("Could not determine if type was celsius or fahrehnheit");
        } else {
            println!("Entered temperature is : {}", temp_string);
        }
    }
}
