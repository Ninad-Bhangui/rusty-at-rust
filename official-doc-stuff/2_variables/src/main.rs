fn main() {
    //Below code will not execute because immutable
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    //Try shadowing
    let x = 5;
    let x = x + 1; //Multiple declarations for same variable name
    let x = x * 2; //Note that this will not give immutable error as it is being redeclared.

    println!("The value of x is: {}", x);
}
