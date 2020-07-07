fn main() {
    let x = 5;
    //Below code will not execute as x is not immutable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
