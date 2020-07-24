/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
*/

fn main() {
    let test_str1 = String::from("did you ever hear the tragedy of darth plagueis the wise");
    println!("{}", piglatin(&test_str1[..]));
}
fn convert(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_letter = word.chars().nth(0).unwrap();
    if vowels.contains(&first_letter) {
        format!("{}-{} ", word, "hay")
    } else {
        /*
        I was wondering why the first line below did not work and why the second line did. After asking on discord, I was told that,
        rust implicitly borrows (does not move). It does so via match foo { ref bar => ... } which requires that the foo (word[1..] here) be Sized.
        To explicitly borrow we have to use &


        My next question was why would rust not know the size of the slice at compile time to which someone answered,
        that one could specify the length it needs to be sliced to at runtime which makes sense.
        */
        //format!("{}-{}{} ", word[1..], first_letter, "ay")
        format!("{}-{}{} ", &word[1..], first_letter, "ay")
    }
}
fn piglatin(sentence: &str) -> String {
    let mut pigstring = String::new();
    for word in sentence.split(" ") {
        let converted = convert(&word[..]);
        pigstring.push_str(&converted);
        println!("{}", pigstring);
    }
    pigstring.trim().to_string()
}

#[test]
// fn test_median() {
//     let test_str1 = String::from("did you ever hear the tragedy of darth plagueis the wise");
//     let result_str1 = String::from("id-day ou-yay ever-hay ear-hay he-tay ragedy-tay of-hay arth-day lagueis-pay he-tay ise-way");

//     assert_eq!(result_str1, piglatin(&test_str1[..]));
// }
#[test]
fn test_convert() {
    let test_str1 = String::from("hello");
    let result_str1 = String::from("ello-hay");
    assert_eq!(result_str1, piglatin(&test_str1[..]));

    let test_str2 = String::from("again");
    let result_str2 = String::from("again-hay");
    assert_eq!(result_str2, piglatin(&test_str2[..]));
}
