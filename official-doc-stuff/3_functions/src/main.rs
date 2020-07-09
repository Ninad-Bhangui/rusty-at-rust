fn main() {
    another_function(5, 6);

    let x = plus_one(6);
    let x = plus_one_alt(6);
    let y = five();

    //let a = 1 is statement. 1 itself is an expression with return value 1. function calls are expressions
    // a+1 is expression.
    let z = {
        let a = 1;
        a + 1
    };

    //Control flow
    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //Unlike python where non zero values evaluate to True, it expects boolean here.
    // if x {
    //     println!("Non boolean condition does not work ");
    // }

    //Functions
    for number in (1..4).rev() {
        println!("{}!", number); //prints 3 2 1 (Last excluded)
    }

    //Loop through elements safely
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
fn plus_one_alt(x: i32) -> i32 {
    return x + 1;
}
//Semicolon at the end indicates it's a statement and not expression. Compiler detects and even recommends removing semicolon
// fn plus_one_invalid(x: i32) -> i32 {
//     x+1;
// }
