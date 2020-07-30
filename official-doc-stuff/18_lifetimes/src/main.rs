fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test_same_scope() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    assert_eq!("abcd", result)
}

#[test]
fn test_use_smaller_scope() {
    /**
     * Here the smaller of the two scopes is used as 'a.
     * scope of string2 is used, string` lies in this scope and so does result
     */
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        assert_eq!("abcd", result)
    }
}

// #[test]
// fn test_result_not_in_smaller_scope() {
//     /**
//      * Here scope of string2 is smaller as above but result lies in the larger scope.assert_eq!
//      * So the function cannot return result
//      * Uncommenting this will throw an error string2.as_str() borrowed value does not live long enough
//      */
//     let string1 = String::from("abcd");
//     let result;
//     {
//         let string2 = String::from("xyz");

//         result = longest(string1.as_str(), string2.as_str());
//     }
//     assert_eq!("abcd", result);
// }

#[test]
fn test_result_in_smallest_scope() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    {
        let result = longest(string1.as_str(), string2.as_str());
        assert_eq!("abcd", result)
    }
}

#[test]
fn test_result_in_larger_scope() {
    let result;
    {
        let string1 = String::from("abcd");
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        assert_eq!("abcd", result)
    }
}
