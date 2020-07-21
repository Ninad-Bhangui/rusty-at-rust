use std::ffi::OsString;
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

    let mut vec2 = vec![1, 2, 3];
    vec2.insert(1, 4);
    println!("vec2 {:?}", vec2);
    // vec2.insert(5, 5); //Panics due to index > length
    // println!("vec2 {:?}", vec2);

    vec2.remove(1); // Removes and shifts. Wondering why pop returns  Options enum but not remove.
    println!("vec2 {:?}", vec2);

    //String Basics
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("{}", s3);
    // println!("{}", s1); //s1 Cannot be used

    let s4 = format!("{} {}", s3, s2);
    println!("{} {} {}", s4, s3, s2); //All can be used

    //Indexing Strings
    let s1 = String::from("hello");
    // let h = &s1[0];  //Does not work

    // Iterating
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    //Extras
    let os_string = OsString::from("foo");
    println!("first letter is {}", os_string[0])
}
