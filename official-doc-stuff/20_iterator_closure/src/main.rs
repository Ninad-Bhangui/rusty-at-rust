fn main() {
    let x = 4;

    let equal_to_x = |z| z == x; //Closure using x works which is not in function scope

    let y = 4;
    println!("{}", x);
    assert!(equal_to_x(y));
    assert!(equal_to_x(y));
}
