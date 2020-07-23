/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
*/

fn main() {
    println!("Hello, world!");
}

fn piglatin(sentence: &str) -> String {
    String::from("test")
}

#[test]
fn test_median() {
    let test_str1 = String::from("did you ever hear the tragedy of darth plagueis the wise?");
    let result_str1 = String::from("id-day ou-yay ever-hay ear-hay he-tay ragedy-tay of-hay arth-day lagueis-pay he-tay ise-way?");
    assert_eq!(String::from("test"), piglatin(&test_str1[..]))
}
