fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{}", s); //
    let s1 = String::from("hello"); // s comes into scope

    let s1 = take_and_return_ownership(s1);
    println!("{}", s1);

    //REFERENCES AND BORROWING
    //Pass reference and ownership is not transferred
    let len = calculate_length(&s1);
    println!("{}", len);

    //Mutable ref
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);

    //Multi mut references
    let r1 = &mut s2; //Not throwing any error now as after r2 is created r1 is never used.
    let r2 = &mut s2;

    // println!("{}", r1);  //This will give an error if uncommented as it would mean two mutable references are being used simultaneously
    println!("{}", r2);

    //One immutable and one mutable ref
    let mut s3 = String::from("hello");

    let r1 = &s3; // no problem
    let r2 = &s3; // no problem
    let r3 = &mut s3; // PROBLEM

    // println!("{}", r1);  //Uncommenting will cause error as immutable and mutable will be used simultaneously
    // println!("{}", r2);
    println!("{}", r3);
}

fn move_ownership() {
    let x = 5;
    let y = x;
    println!("{}", x); //Works. Both are valid.

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); //Error. s1 is no longer valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn take_and_return_ownership(some_string: String) -> String {
    some_string
}
fn calculate_length(s: &String) -> usize {
    //Does not have ownership so never dropped
    s.len()
}
fn change(s: &mut String) {
    s.push_str(" world");
}
