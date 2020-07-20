fn main() {
    //Basic docs
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
    println!("{:?}", v);

    //Extras
    assert_eq!(v.len(), 2); //Length
    assert_eq!(v[0], 55); //Access

    assert_eq!(vec!(55, 56), v); //Initialization using macros
    assert_eq!(vec![0; 5], vec!(0, 0, 0, 0, 0));
    let mut vec1 = Vec::with_capacity(5);
    vec1.resize(5, 0);
    assert_eq!(vec![0; 5], vec1);
    let a = v.pop(); //Returns Options enum instance probably because an empty array would have nothing to pop and return None variant
    println!("Popped item : {:?}", a);
}
