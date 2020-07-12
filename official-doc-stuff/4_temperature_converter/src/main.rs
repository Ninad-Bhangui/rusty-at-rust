use std::io;

fn main() {
    loop {
        println!("Type in the temperature in the format 10F/100C (Fahrehnheit/Celsius). Press q/Q to exit,");

        let temp_string = {
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

        let (temperature, scale) = temp_string.split_at(temp_string.len() - 1);
        let temperature: f32 = match temperature.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid format!");
                continue;
            }
        };
        let converted_temp = match scale {
            "C" => convert(temperature, scale),
            "F" => convert(temperature, scale),
            _ => {
                println!("Invalid scale");
                continue
            }
        };
        println!("Converted temperature is : {}", converted_temp);
    }
}

fn convert(temperature: f32, scale: &str) -> f32 {
    match scale {
        "C" => temperature * 9.0 / 5.0 + 32.0,
        "F" => (temperature - 32.0) * 5.0 / 9.0,
        //This part of code is weird. I am forced to return a float unlike python. Will return to improve this
        _ => {
            println!("Invalid scale");
            temperature
        }

    }
}
//Tried writing basic unit test. Will look into structuring it when that point is reached in docs.
#[test] fn test_converter(){
    assert_eq!(32.0,convert(0.0, "C"));
    assert_eq!(0.0,convert(32.0, "F"));
}