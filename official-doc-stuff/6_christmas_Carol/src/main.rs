fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    for (index, day) in days.iter().enumerate() {
        println!("On the {} day of christmas my true love sent to me", day);
        for i in (0..index + 1).rev() {
            if i == 0 && index != 0 {
                println!("and {}", gifts[i]);
            } else {
                println!("{}", gifts[i]);
            }
        }
        println!("\n");
    }
}
