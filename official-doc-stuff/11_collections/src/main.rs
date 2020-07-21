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

    //Hash Map
    use std::collections::hash_map;
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // ZIP creates vector of tuples
    // Collect is able to infer that it should collect into a hashmap based on type annotation
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);

    //Ownership
    let key = String::from("five");
    let val = 5;
    let key2 = String::from("six");
    let val2 = 6;
    let mut map = HashMap::new();
    map.insert(key, val);
    let mut map2 = HashMap::new();
    map2.insert(&key2, val2);

    // println!("{}", key); //Does not compile as String is borrowed
    println!("{}", val); //Compile as not borrowed

    println!("{:?}", map2);
    println!("{}", key2); //key2 can be used as reference was passed to map2

    let key3 = key2;
    // println!("{:?}", map2);  //This line would throw an error since key2 gets moved above while map2 still has a refernece to it.

    //Iterating over HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //Iterating over Mutable HashMap
    for (key, value) in &mut scores {
        *value += 100;
    }

    scores.insert(String::from("Blue"), 999);

    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(0); // Does not update
    scores.entry(String::from("Green")).or_insert(33); // Updates

    println!("{:?}", scores);

    //.entry returns an enum so let's try match with it

    match scores.entry(String::from("Blue")) {
        hash_map::Entry::Occupied(s) => println!(" {:?} Key exists!", s),
        hash_map::Entry::Vacant(s) => println!(" {:?} Key does not exist", s),
    }

    match scores.entry(String::from("Black")) {
        hash_map::Entry::Occupied(s) => println!("{:?} Key exists!", s),
        hash_map::Entry::Vacant(s) => println!(" {:?} Key does not exist", s),
    }

    //Insert returns a mutable reference. SO let's play with that

    let a = scores.entry(String::from("Blue")).or_insert(0); // Does not update
    let b = scores.entry(String::from("Orange")).or_insert(44); // Updates

    //TODO: Uncommenting this causes error. Investigate
    // println!("{}", a);
    println!("{}", b);
}
