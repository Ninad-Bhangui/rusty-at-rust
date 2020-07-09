fn main() {
    let x = 5;
    //Below code will not execute as x is not immutable
    // println!("The value of x is: {}", x);
    // x = 6;
    println!("The value of x is: {}", x);

    //Below code will panic on cargo run but will perform two's complement wrapping on cargo run --release and value would be 0
    // let mut overflow: u8 = 255;
    // overflow = overflow + 1;
    // println!("The value of overflow is: {}", overflow);

    //Arithematic that works

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "Addition : {}, Difference : {}, Product : {}, Quotient : {}, Remainder : {}",
        sum, difference, product, quotient, remainder
    );

    //Trying two mismatched types : no implementation for `{integer} + {float}`
    // let mismatch_sum = 5 + 34.0;
    // to add either convert integer to float
    // let mismatch_sum = f64::from(5) + 34.0;

    //Difference between as and from::()
    //https://stackoverflow.com/questions/48795329/what-is-the-difference-between-fromfrom-and-as-in-rust
    // let mismatch_sum = 5 as f64 + 34.0;
    //Or float to integer
    // let mismatch_sum = 5 + 34.0 as i32;
    // println!("Mismatched sum is : {}", mismatch_sum);

    //Tuple
    let g = (500, 6.4, 1);
    let g: (i32, f64, u8) = (500, 6.4, 1);
    let e = g.2;

    //Array
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
