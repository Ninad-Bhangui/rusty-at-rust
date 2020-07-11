use std::io;

fn main() {
    loop {
        println!("Type in the temperature in the format 10F/100C (Fahrehnheit/Celsius). Press q/Q to exit,");

        let mut temp_string = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read line!");

            //trim returns string slice and not string. If the return was input.trim() it would throw an error as input goes out of scope when used later
            //input.to_uppercase().trim()   //Throws error
            input.to_uppercase().trim().to_string()
        };
        if temp_string == "Q" {
            println!("Exiting..");
            break;
        }

        let scale = temp_string.split_off(temp_string.len() - 1);
        let temp_string: f32 = match temp_string.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid format!");
                continue;
            }
        };
        if scale == "C" || scale == "F" {
            let converted_temp = convert(temp_string, scale);

            println!("Converted temperature is : {}", converted_temp);
        } else {
            println!("Could not determine Scale")
        }
    }
}

fn convert(temperature: f32, scale: String) -> f32 {
    if scale == "C" {
        return temperature * 9.0 / 5.0 + 32.0;
    } else if scale == "F" {
        return (temperature - 32.0) * 5.0 / 9.0;
    } else {
        return temperature;
    }
}