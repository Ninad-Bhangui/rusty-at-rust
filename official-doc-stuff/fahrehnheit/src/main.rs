use std::io;
fn main() {
    println!("Hello, world!");
    println!("Enter temperature in Celsius : ");
    let mut temp =  String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line!!");
    let temp: f64 = temp.trim().parse().expect("Please type a number");
    let fah_temp = convert(temp);
    println!("Your temperature in Fahrehnheit is : {}", fah_temp);

}

fn convert(cel: f64) -> f64{
    let _conversion_rate: f64 =9.0/5.0;
    let _offset: f64 = 32.0;
    return cel*_conversion_rate + _offset;

}