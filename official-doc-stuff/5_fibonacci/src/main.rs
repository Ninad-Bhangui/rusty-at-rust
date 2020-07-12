use std::io;
fn main() {
    println!("Enter any positive integer n to generate fibonacci sequence");
    let n: i32 = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        input
            .trim()
            .parse()
            .expect("Could not parse integer from input")
    };

    println!("{}", fibo_gen(n));
}

fn fibo_gen(n: i32) -> String {
    let mut f0 = 0;
    let mut f1 = 1;
    let mut fibo_str = format!("{}, {}", f0, f1); //format macro can be used for string interpolation
    let mut i = 1;
    while i < n {
        let f_latest = f1 + f0;
        f0 = f1;
        f1 = f_latest;
        // fibo_str.push_str(&format!(", {}", number)); //push_str appends &str to String so we have to pass a reference as format returns String
        fibo_str = format!("{}, {}", fibo_str, f_latest);
        i += 1;
    }
    fibo_str
}
//Tried writing basic unit test. Will look into structuring it when that point is reached in docs.
#[test]
fn test_converter() {
    assert_eq!("0, 1, 1, 2", fibo_gen(3));
    assert_eq!("0, 1, 1, 2, 3, 5", fibo_gen(5));
    assert_eq!(
        "0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377",
        fibo_gen(14)
    );
}
