# Exercise 1

## Convert between fahrehnheit and celsius

Following are important snippets of the code.

Found someone else's code which was better structured but uses concepts not covered yet.
<https://benjaminbrandt.com/converting-temperatures-in-rust/>

### Take user input

```rust
        let temp_string = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read line!");

            //trim returns string slice and not string. If the return was input.trim() it would throw an error as input goes out of scope when used later
            //input.to_uppercase().trim()   //Throws error
            input.to_uppercase().trim().to_string()
        };
```

User input is taken in the format `{value}{scale}`. For example, `32F` or `100C`

### Parse user input into temperature and scale

```rust
        let (temperature, scale) = temp_string.split_at(temp_string.len() - 1);
        let temperature: f32 = match temperature.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid format!");
                continue;
            }
        };
```

### Conversion function

```rust
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
```
