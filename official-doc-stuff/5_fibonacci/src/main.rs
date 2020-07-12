use std::io;
fn main() {
    println!("Enter any positive integer n to generate fibonacci sequence");
    let n: u32 = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        input
            .trim()
            .parse()
            .expect("Could not parse integer from input")
    };

    //println!("{}", fibo_seq_gen(n));
    println!("{}", fibo_gen(n));
}
fn fibo_gen(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo_gen(n - 1) + fibo_gen(n - 2),
    }
}
//Accidentally wrote a sequence generator but we just need the nth fibonacci number
fn fibo_seq_gen(n: u32) -> String {
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
//Tried writing basic unit test for sequence. Will look into structuring it when that point is reached in docs.
#[test]
fn test_fibo_sequence() {
    assert_eq!("0, 1, 1, 2", fibo_seq_gen(3));
    assert_eq!("0, 1, 1, 2, 3, 5", fibo_seq_gen(5));
    assert_eq!(
        "0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377",
        fibo_seq_gen(14)
    );
}

#[test]
fn test_fibo() {
    assert_eq!(0, fibo_gen(0));
    assert_eq!(1, fibo_gen(1));
    assert_eq!(1, fibo_gen(2));
    assert_eq!(2, fibo_gen(3));
    assert_eq!(3, fibo_gen(4));
    assert_eq!(5, fibo_gen(5));
    assert_eq!(8, fibo_gen(6));
    assert_eq!(13, fibo_gen(7));
    assert_eq!(21, fibo_gen(8));
    assert_eq!(34, fibo_gen(9));
    assert_eq!(55, fibo_gen(10));
    assert_eq!(89, fibo_gen(11));
    assert_eq!(144, fibo_gen(12));
    assert_eq!(233, fibo_gen(13));
    assert_eq!(377, fibo_gen(14));
}
