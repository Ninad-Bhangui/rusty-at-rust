fn main() {
    println!("Hello, world!");
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    // let third: &i32 = &v[2]; //Panics
    match v.get(1) {
        Some(third) => println!("The second element is {}", third),
        None => println!("There is no second element."),
    };
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    };
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v)
}
